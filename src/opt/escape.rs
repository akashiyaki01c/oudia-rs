//! OuPropertiesText用に文字列にエスケープ処理を施すモジュール

/// 文字列をエスケープする関数
pub fn escape_text(text: String) -> String {
    text.replace("\\", "\\\\").replace("\n", "\\n")
}

/// 文字列のエスケープを解除する関数
pub fn unescape_text(text: String) -> String {
	text.replace("\\n", "\n").replace("\\\\", "\\")
}
