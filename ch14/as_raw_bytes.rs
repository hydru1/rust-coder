/// 変数をバイト列で表したときの中身を&[u8]型で返す関数を実装
/// 参考: https://qiita.com/Kogia_sima/items/88920a2a14448ef4dbe3

trait AsRawBytes {
    fn as_raw_bytes<'a>(&'a self) -> &'a [u8];
}

impl<T: ?Sized> AsRawBytes for T {
    fn as_raw_bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const T as *const u8,
                std::mem::size_of_val(self))
        }
    }
}
