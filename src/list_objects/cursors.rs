use crate::{
    IoCtx, Result,
    error::maybe_err_or_val,
    librados::{rados_object_list, rados_object_list_slice},
    list_objects::{Object, cursors::cursor::ListCursor},
};

impl IoCtx<'_> {
    /// Create a cursor that will yield all objects in the pool and
    /// namespace configured on this [`IoCtx`].
    ///
    /// Note that filtering based on the namespace occurs synchronously,
    /// so yielding from the returned [`Cursor`] may take a long time.
    pub fn object_cursor(&self) -> Cursor<'_, '_> {
        Cursor::new(self)
    }
}

/// An object cursor.
///
/// Objects yielded by this cursor are synchronously filtered
/// by the namespace configured on the passed-in [`IoCtx`].
#[derive(Debug)]
pub struct Cursor<'ioctx, 'rados> {
    io: &'ioctx IoCtx<'rados>,
    start: ListCursor<'ioctx, 'rados>,
    end: ListCursor<'ioctx, 'rados>,
}

impl Clone for Cursor<'_, '_> {
    fn clone(&self) -> Self {
        let cloned = self.split(1).next().expect("Failed to split once");
        cloned
    }
}

impl<'ioctx, 'rados> Cursor<'ioctx, 'rados> {
    pub(crate) fn new(io: &'ioctx IoCtx<'rados>) -> Self {
        let start = ListCursor::begin(io);
        let end = ListCursor::end(io);

        Self { io, start, end }
    }

    /// Reset this cursor to the start of the pool,
    /// causing all previously yielded items to be
    /// yielded again.
    pub fn reset(&mut self) {
        self.start = ListCursor::begin(self.io);
    }

    /// Attempt to read the next `n` entries into a [`Vec`] and advance
    /// the underlying cursor past the read objects.
    ///
    /// May return fewer than `n` entries if the end of the pool has
    /// been reached.
    pub fn read<'me>(&'me mut self, n: usize) -> Result<Vec<Object<'me>>> {
        let mut results: Vec<Object> = Vec::with_capacity(n);

        self.read_into(&mut results)?;

        Ok(results)
    }

    /// Read up to `results.capacity()` entries into `results` and
    /// advance the underlying cursor past the read objects.
    ///
    /// Once this function returns, `results` will contain the read
    /// objects. If fewer than `results.capacity()` objects were left
    /// for reading, the amount of values contained in `results` will
    /// be reduced.
    ///
    /// Note that the allocations performed by the program itself
    /// are relatively small: using `read_into` will only have minor
    /// performance benefits over [`Cursor::read`].
    ///
    //
    // Implementation note: this cannot take `&mut [Object]` because
    // `Object` is not externally constructable (yet). `rados_object_list`
    // zeroes out all of the values in `results` which could cause null-ptrs to be
    // contained in some of the resulting `Object`s, which we _really_ don't want
    // to expose to end users. This is just a slightly easier approach for now.
    pub fn read_into<'me>(&'me mut self, results: &mut Vec<Object<'me>>) -> Result<()> {
        // Truncate to ensure that all items in the
        // list are dropped (even though they shouldn't
        // really need it)
        results.truncate(0);

        let mut next = ListCursor::begin(self.io);

        let result_count = maybe_err_or_val(unsafe {
            rados_object_list(
                self.io.inner(),
                self.start.inner,
                self.end.inner,
                results.capacity(),
                std::ptr::null(),
                0,
                results.as_mut_ptr() as _,
                &mut next.inner,
            )
        })?;

        self.start = next;

        unsafe { results.set_len(result_count as _) };

        Ok(())
    }

    /// Split this cursor into `chunks` cursors, each representing a slice
    /// whose size is a fraction close to `1/chunks` of all objects
    /// that will eventually be yielded by `self`.
    pub fn split(&self, chunks: usize) -> impl Iterator<Item = Cursor<'ioctx, 'rados>> {
        struct Iter<'cur, 'ioctx, 'rados> {
            inner: &'cur Cursor<'ioctx, 'rados>,
            m: usize,
            n: usize,
        }

        impl<'ioctx, 'rados> Iterator for Iter<'_, 'ioctx, 'rados> {
            type Item = Cursor<'ioctx, 'rados>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.n >= self.m {
                    return None;
                }

                let io = self.inner.io;
                let mut split_start = ListCursor::begin(io);
                let mut split_finish = ListCursor::begin(io);

                unsafe {
                    rados_object_list_slice(
                        io.inner(),
                        self.inner.start.inner,
                        self.inner.end.inner,
                        self.n,
                        self.m,
                        &mut split_start.inner,
                        &mut split_finish.inner,
                    )
                };

                assert!(!split_start.inner.is_null());
                assert!(!split_finish.inner.is_null());

                let new = Cursor {
                    io: self.inner.io,
                    start: split_start,
                    end: split_finish,
                };

                self.n += 1;

                Some(new)
            }
        }

        Iter {
            inner: self,
            m: chunks,
            n: 0,
        }
    }
}

mod cursor {
    use crate::{
        IoCtx,
        librados::{
            rados_object_list_begin, rados_object_list_cursor, rados_object_list_cursor_free,
            rados_object_list_end, rados_object_list_is_end,
        },
    };

    #[derive(Debug)]
    pub(super) struct ListCursor<'ioctx, 'rados> {
        pub io: &'ioctx IoCtx<'rados>,
        pub inner: rados_object_list_cursor,
    }

    unsafe impl Send for ListCursor<'_, '_> {}
    unsafe impl Sync for ListCursor<'_, '_> {}

    impl<'ioctx, 'rados> ListCursor<'ioctx, 'rados> {
        pub fn begin(io: &'ioctx IoCtx<'rados>) -> Self {
            let inner = unsafe { rados_object_list_begin(io.inner()) };
            Self { io, inner }
        }

        pub fn end(io: &'ioctx IoCtx<'rados>) -> Self {
            let inner = unsafe { rados_object_list_end(io.inner()) };
            Self { io, inner }
        }

        #[allow(unused)]
        pub fn is_end(&self) -> bool {
            let res = unsafe { rados_object_list_is_end(self.io.inner(), self.inner) };

            res == 1
        }
    }

    impl Drop for ListCursor<'_, '_> {
        fn drop(&mut self) {
            unsafe { rados_object_list_cursor_free(self.io.inner(), self.inner) };
        }
    }
}
