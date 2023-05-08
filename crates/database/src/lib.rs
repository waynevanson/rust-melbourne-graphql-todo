use petgraph::{
    prelude::{DiGraph, EdgeIndex, NodeIndex},
    visit::IntoNodeReferences,
};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type Database = DiGraph<Entity, Relationship>;

#[derive(Debug, Default)]
pub enum Door {
    Opened,
    #[default]
    Closed,
}

pub struct GraphDB {
    lock: RwLock<Database>,
}

impl Default for GraphDB {
    fn default() -> Self {
        let lock = RwLock::default();
        Self { lock }
    }
}

impl GraphDB {
    fn read(&self) -> RwLockReadGuard<Database> {
        self.lock.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<Database> {
        self.lock.write().unwrap()
    }

    pub fn find_persons(&self) -> Vec<Person> {
        self.read()
            .node_weights()
            .filter_map(|node_weight| match node_weight {
                Entity::Person(person) => Some(person),
                _ => None,
            })
            .cloned()
            .collect()
    }

    pub fn person_ids(&self) -> Vec<String> {
        self.read()
            .node_weights()
            .filter_map(|node_weight| match node_weight {
                Entity::Person(person) => Some(person),
                _ => None,
            })
            .map(|user| user.id.to_owned())
            .collect()
    }

    pub fn find_person(&self, id: impl ToString) -> Option<Person> {
        self.read()
            .node_weights()
            .find_map(|node_weight| match node_weight {
                Entity::Person(person) if id.to_string() == person.id => Some(person),
                _ => None,
            })
            .cloned()
    }

    pub fn insert_person(&self, person: Person) -> NodeIndex {
        let mut graph = self.write();
        graph.add_node(Entity::Person(person))
    }

    pub fn insert_facility(&self, facility: Facility) -> NodeIndex {
        let mut graph = self.write();
        graph.add_node(Entity::Facility(facility))
    }

    pub fn insert_person_in_facility(&self, person: Person, facility: Facility) -> EdgeIndex {
        let node_person = self.insert_person(person);
        let node_facility = self.insert_facility(facility);

        let mut graph = self.write();
        let weight = Relationship::Participant;
        graph.add_edge(node_person, node_facility, weight)
    }

    pub fn move_person_to_facility(
        &self,
        person_id: String,
        facility_type: Option<FacilityType>,
    ) -> Option<()> {
        let mut graph = self.write();

        let node_index = graph
            .node_references()
            .find_map(|(node_index, node_weight)| match node_weight {
                Entity::Person(person) if person_id == person.id => Some(node_index),
                _ => None,
            })
            .expect("Cannot find person to move");

        let neighbors = graph
            .neighbors(node_index)
            .filter_map(|node| graph.find_edge(node_index, node))
            .collect::<Vec<_>>();

        for edge in neighbors {
            graph.remove_edge(edge);
        }

        facility_type
            .and_then(|facility_type| {
                graph
                    .node_references()
                    .find_map(|(node_index, node_weight)| match node_weight {
                        Entity::Facility(facility) if facility.type_ == facility_type => {
                            Some(node_index)
                        }
                        _ => None,
                    })
            })
            .map(|facility_index| {
                graph.add_edge(node_index, facility_index, Relationship::Participant);
            })
    }
}

#[derive(Debug, Clone)]
pub enum Entity {
    Person(Person),
    Facility(Facility),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Relationship {
    Participant,
}

#[derive(Debug, Clone)]
pub struct Person {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone)]
pub struct Facility {
    pub id: String,
    pub type_: FacilityType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FacilityType {
    Sauna,
    SteamRoom,
    Spa,
}
