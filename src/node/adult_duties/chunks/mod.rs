// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

mod chunk_storage;
mod reading;
mod writing;

use crate::{network::Routing, node::node_ops::MessagingDuty, node::state_db::NodeInfo, Result};
use chunk_storage::ChunkStorage;
use reading::Reading;
use writing::Writing;

use log::trace;
use safe_nd::{Cmd, DataCmd, DataQuery, Message, MsgEnvelope, Query};

use std::{
    cell::Cell,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

/// Operations on data chunks.
pub(crate) struct Chunks<R: Routing + Clone> {
    chunk_storage: ChunkStorage<R>,
}

impl<R: Routing + Clone> Chunks<R> {
    pub fn new(node_info: NodeInfo<R>, total_used_space: &Rc<Cell<u64>>) -> Result<Self> {
        let chunk_storage = ChunkStorage::new(node_info, total_used_space)?;

        Ok(Self { chunk_storage })
    }

    pub fn receive_msg(&mut self, msg: &MsgEnvelope) -> Option<MessagingDuty> {
        trace!(
            "{}: Received ({:?} from src {:?}",
            self,
            msg.id(),
            msg.most_recent_sender().address(),
        );
        match &msg.message {
            Message::Query {
                query: Query::Data(DataQuery::Blob(read)),
                ..
            } => {
                let reading = Reading::new(read.clone(), msg.clone());
                reading.get_result(&self.chunk_storage)
            }
            Message::Cmd {
                cmd:
                    Cmd::Data {
                        cmd: DataCmd::Blob(write),
                        ..
                    },
                ..
            } => {
                let writing = Writing::new(write.clone(), msg.clone());
                writing.get_result(&mut self.chunk_storage)
            }
            _ => None,
        }
    }

    // fn handle_response(
    //     &mut self,
    //     src: SrcLocation,
    //     response: Response,
    //     requester: PublicId,
    //     message_id: MessageId,
    //     proof: Option<(Request, Signature)>,
    // ) -> Option<MessagingDuty> {
    //     use Response::*;
    //     trace!(
    //         "{}: Received ({:?} {:?}) from {}",
    //         self,
    //         response,
    //         message_id,
    //         utils::get_source_name(src),
    //     );
    //     if let Some((request, signature)) = proof {
    //         if !matches!(requester, PublicId::Node(_))
    //             && self
    //                 .validate_section_signature(&request, &signature)
    //                 .is_none()
    //         {
    //             error!("Invalid section signature");
    //             return None;
    //         }
    //         match response {
    //             GetBlob(result) => {
    //                 if matches!(requester, PublicId::Node(_)) {
    //                     debug!("got the duplication copy");
    //                     if let Ok(data) = result {
    //                         trace!(
    //                             "Got GetBlob copy response for address: ({:?})",
    //                             data.address(),
    //                         );
    //                         self.chunk_storage.store(
    //                             src,
    //                             &data,
    //                             &requester,
    //                             message_id,
    //                             Some(&signature),
    //                             request,
    //                         )
    //                     } else {
    //                         None
    //                     }
    //                 } else {
    //                     None
    //                 }
    //             }
    //             //
    //             // ===== Invalid =====
    //             //
    //             ref _other => {
    //                 error!(
    //                     "{}: Should not receive {:?} as a data handler.",
    //                     self, response
    //                 );
    //                 None
    //             }
    //         }
    //     } else {
    //         error!("Missing section signature");
    //         None
    //     }
    // }

    // fn public_key(&self) -> Option<PublicKey> {
    //     Some(
    //         self.routing_node
    //             .borrow()
    //             .public_key_set()
    //             .ok()?
    //             .public_key(),
    //     )
    // }

    // fn validate_section_signature(&self, request: &Request, signature: &Signature) -> Option<()> {
    //     if self
    //         .public_key()?
    //         .verify(signature, &utils::serialise(request))
    //     {
    //         Some(())
    //     } else {
    //         None
    //     }
    // }
}

impl<R: Routing + Clone> Display for Chunks<R> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Chunks")
    }
}
