#[inline(always)]
pub(crate) fn new_view(src: &[u8], offset: usize, length: usize) -> &[u8] {
  &src[offset..offset + length]
}

#[inline(always)]
pub(crate) fn read_to_string(src: &[u8]) -> String {
  String::from_utf8_lossy(src).to_string()
}
