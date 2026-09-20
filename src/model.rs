pub mod oudia102;
pub mod error;

/// パースされたRosenFileDataを表す。
/// バージョンによって差異があることから、バージョンごとに列挙子を割り当てる。
pub enum OuDiaFile {
	/// `OuDia.1.02`形式
	OuDia102(oudia102::RosenFileData),
}
