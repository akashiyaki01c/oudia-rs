pub mod oudia102;
pub mod error;
pub mod oudiasecond100;

/// パースされたRosenFileDataを表す。
/// バージョンによって差異があることから、バージョンごとに列挙子を割り当てる。
pub enum OuDiaFile {
	/// `OuDia.1.02`形式
	OuDia102(oudia102::RosenFileData),
	/// `OuDiaSecond.1.00`形式
	OuDiaSecond100(oudiasecond100::RosenFileData),
}
