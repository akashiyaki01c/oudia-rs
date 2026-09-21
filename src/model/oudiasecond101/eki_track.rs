use crate::{model::error::Error, opt::{directory::Directory, node::Node, property::Property}};

const KEY_EKI_TRACK: &str = "EkiTrack2";
const KEY_TRACK_NAME: &str = "TrackName";
const KEY_TRACK_RYAKUSYOU: &str = "TrackRyakusyou";

/// 1つの駅番線を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EkiTrack {
    /// 番線名
    track_name: String,
    /// 番線の略称名
	ryakusyou: String,
}
impl EkiTrack {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // TrackName
            if let Some(name) = dir.find(KEY_TRACK_NAME) {
                if let Node::Property(name) = name {
                    result.track_name = name.value.clone();
                }
            }

			// TrackRyakusyou
            if let Some(ryakusyou) = dir.find(KEY_TRACK_RYAKUSYOU) {
                if let Node::Property(ryakusyou) = ryakusyou {
                    result.ryakusyou = ryakusyou.value.clone();
                }
            }

            
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        let values = vec![
            property(KEY_TRACK_NAME, &self.track_name),
			property(KEY_TRACK_RYAKUSYOU, &self.ryakusyou),
            
        ];
        
        Node::Directory(Directory::new_with_value(KEY_EKI_TRACK, values))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}