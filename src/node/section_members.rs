// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use routing::Node as Routing;
use safe_nd::{Address, Cmd, DataCmd, Duty, ElderDuty, Message, MsgEnvelope, MsgSender, XorName};

#[derive(Clone)]
pub(crate) struct SectionMembers {
    routing: Rc<RefCell<Routing>>,
}

impl SectionMembers {

    pub fn new(routing: Rc<RefCell<Routing>>) -> Self {
        Self {
            routing
        }
    }

    pub fn our_adults_sorted_by_distance_to(&self, name: &XorName, count: usize) -> Vec<&XorName> {
        self.routing
                .our_elders_sorted_by_distance_to(&routing::XorName(name.0))
                .into_iter()
                .take(count)
                .map(|p2p_node| XorName(p2p_node.name().0))
                .collect::<Vec<_>>()
    }

    pub fn our_elders_sorted_by_distance_to(&self, name: &XorName, count: usize) -> Vec<&XorName> {
        routing
                .our_elders_sorted_by_distance_to(&routing::XorName(target.0))
                .into_iter()
                .take(count)
                .map(|p2p_node| XorName(p2p_node.name().0))
                .collect::<Vec<_>>()
    }
}