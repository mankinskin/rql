use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use hashbrown::hash_map;

use crate::Id;

/// A row in a `Table`
pub struct Row<'a, T> {
    id: Id<T>,
    data: &'a T,
}

impl<'a, T> Row<'a, T> {
    /// Change the data held in a row without changing its id
    pub fn map<F, U>(&self, f: F) -> MappedRow<T, U>
    where
        F: Fn(&T) -> U,
    {
        MappedRow {
            id: self.id,
            data: f(self.data),
        }
    }
}

impl<'a, T> Clone for Row<'a, T> {
    fn clone(&self) -> Self {
        Row {
            id: self.id,
            data: self.data,
        }
    }
}

impl<'a, T> Copy for Row<'a, T> {}

impl<'a, T, U> PartialEq<Row<'a, U>> for Row<'a, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Row<U>) -> bool {
        self.data.eq(&other.data)
    }
}

impl<'a, T> Eq for Row<'a, T> where T: Eq {}

impl<'a, T, U> PartialEq<RowMut<'a, U>> for Row<'a, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &RowMut<U>) -> bool {
        self.data.eq(other.data)
    }
}

impl<'a, T, U, V> PartialEq<MappedRow<V, U>> for Row<'a, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &MappedRow<V, U>) -> bool {
        self.data.eq(&other.data)
    }
}

impl<'a, T> Deref for Row<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> AsRef<T> for Row<'a, T> {
    fn as_ref(&self) -> &T {
        self.data
    }
}

impl<'a, T> fmt::Debug for Row<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}: {:#?}", self.id, self.data)
        } else {
            write!(f, "{}: {:?}", self.id, self.data)
        }
    }
}

impl<'a, T> fmt::Display for Row<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <T as fmt::Display>::fmt(&self.data, f)
    }
}

/// A mutable row in a `Table`
pub struct RowMut<'a, T> {
    id: Id<T>,
    data: &'a mut T,
}

impl<'a, T> RowMut<'a, T> {
    /// Change the data held in a row without changing its id
    pub fn map<F, U>(&self, f: F) -> MappedRow<T, U>
    where
        F: Fn(&T) -> U,
    {
        MappedRow {
            id: self.id,
            data: f(self.data),
        }
    }
}

impl<'a, T, U> PartialEq<RowMut<'a, U>> for RowMut<'a, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &RowMut<U>) -> bool {
        self.data.eq(&other.data)
    }
}

impl<'a, T> Eq for RowMut<'a, T> where T: Eq {}

impl<'a, T, U> PartialEq<Row<'a, U>> for RowMut<'a, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Row<U>) -> bool {
        (self.data as &T).eq(other.data)
    }
}

impl<'a, T, U, V> PartialEq<MappedRow<V, U>> for RowMut<'a, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &MappedRow<V, U>) -> bool {
        (self.data as &T).eq(&other.data)
    }
}

impl<'a, T> Deref for RowMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> DerefMut for RowMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, T> AsRef<T> for RowMut<'a, T> {
    fn as_ref(&self) -> &T {
        self.data
    }
}

impl<'a, T> AsMut<T> for RowMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T> fmt::Debug for RowMut<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}: {:#?}", self.id, self.data)
        } else {
            write!(f, "{}: {:?}", self.id, self.data)
        }
    }
}

impl<'a, T> fmt::Display for RowMut<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <T as fmt::Display>::fmt(&self.data, f)
    }
}

/// A row that was mapped from another and owns its data
pub struct MappedRow<I, T> {
    id: Id<I>,
    data: T,
}

impl<I, T> MappedRow<I, T> {
    /// Change the data held in a row without changing its id
    pub fn map<F, U>(self, f: F) -> MappedRow<I, U>
    where
        F: Fn(T) -> U,
    {
        MappedRow {
            id: self.id,
            data: f(self.data),
        }
    }
}

impl<I, T> Clone for MappedRow<I, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        MappedRow {
            id: self.id,
            data: self.data.clone(),
        }
    }
}

impl<I, T> Copy for MappedRow<I, T> where T: Copy {}

impl<I, T, U> PartialEq<MappedRow<I, U>> for MappedRow<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &MappedRow<I, U>) -> bool {
        self.data.eq(&other.data)
    }
}

impl<'a, T, U, V> PartialEq<Row<'a, U>> for MappedRow<V, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Row<U>) -> bool {
        self.data.eq(other.data)
    }
}

impl<'a, T, U, V> PartialEq<RowMut<'a, U>> for MappedRow<V, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &RowMut<U>) -> bool {
        self.data.eq(other.data)
    }
}

impl<I, T> Deref for MappedRow<I, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I, T> AsRef<T> for MappedRow<I, T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<I, T> AsMut<T> for MappedRow<I, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<I, T> DerefMut for MappedRow<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<I, T> fmt::Debug for MappedRow<I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}: {:#?}", self.id, self.data)
        } else {
            write!(f, "{}: {:?}", self.id, self.data)
        }
    }
}

impl<I, T> fmt::Display for MappedRow<I, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <T as fmt::Display>::fmt(&self.data, f)
    }
}

/// An iterator over rows in a `Table`
#[derive(Debug)]
pub struct RowIter<'a, T> {
    pub(crate) inner: hash_map::Iter<'a, Id<T>, T>,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = Row<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, data)| Row { id: *id, data })
    }
}

impl<'a, T> Clone for RowIter<'a, T> {
    fn clone(&self) -> Self {
        RowIter {
            inner: self.inner.clone(),
        }
    }
}

/// An mutable iterator over rows in a `Table`
#[derive(Debug)]
pub struct RowIterMut<'a, T> {
    pub(crate) inner: hash_map::IterMut<'a, Id<T>, T>,
}

impl<'a, T> Iterator for RowIterMut<'a, T> {
    type Item = RowMut<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, data)| RowMut { id: *id, data })
    }
}
