use std::io::Cursor;
use std::cmp::PartialEq;

use FromCursor;
use types::{CString, CStringList, CInet};

// Event types
const TOPOLOGY_CHANGE: &'static str = "TOPOLOGY_CHANGE";
const STATUS_CHANGE: &'static str = "STATUS_CHANGE";
const SCHEMA_CHANGE: &'static str = "SCHEMA_CHANGE";

// Topologe changes
const NEW_NODE: &'static str = "NEW_NODE";
const REMOVED_NODE: &'static str = "REMOVED_NODE";

// Status changes
const UP: &'static str = "UP";
const DOWN: &'static str = "DOWN";

// Schema changes
const CREATED: &'static str = "CREATED";
const UPDATED: &'static str = "UPDATED";
const DROPPED: &'static str = "DROPPED";

// Schema change targets
const KEYSPACE: &'static str = "KEYSPACE";
const TABLE: &'static str = "TABLE";
const TYPE: &'static str = "TYPE";
const FUNCTION: &'static str = "FUNCTION";
const AGGREGATE: &'static str = "AGGREGATE";


/// Simplified `ServerEvent` that does not contain details
/// about a concrete change. It may be useful for subscription
/// when you need only string representation of an event.
#[derive(Debug, PartialEq)]
pub enum SimpleServerEvent {
    TopologyChange,
    StatusChange,
    SchemaChange,
}

impl SimpleServerEvent {
    pub fn as_string(&self) -> String {
        match self {
            &SimpleServerEvent::TopologyChange => String::from(TOPOLOGY_CHANGE),
            &SimpleServerEvent::StatusChange => String::from(STATUS_CHANGE),
            &SimpleServerEvent::SchemaChange => String::from(SCHEMA_CHANGE),
        }
    }
}

impl From<ServerEvent> for SimpleServerEvent {
    fn from(event: ServerEvent) -> SimpleServerEvent {
        match event {
            ServerEvent::TopologyChange(_) => SimpleServerEvent::TopologyChange,
            ServerEvent::StatusChange(_) => SimpleServerEvent::TopologyChange,
            ServerEvent::SchemaChange(_) => SimpleServerEvent::TopologyChange,
        }
    }
}

impl<'a> From<&'a ServerEvent> for SimpleServerEvent {
    fn from(event: &'a ServerEvent) -> SimpleServerEvent {
        match event {
            &ServerEvent::TopologyChange(_) => SimpleServerEvent::TopologyChange,
            &ServerEvent::StatusChange(_) => SimpleServerEvent::TopologyChange,
            &ServerEvent::SchemaChange(_) => SimpleServerEvent::TopologyChange,
        }
    }
}

impl PartialEq<ServerEvent> for SimpleServerEvent {
    fn eq(&self, full_event: &ServerEvent) -> bool {
        self == &SimpleServerEvent::from(full_event)
    }
}

/// Full server event that contains all details about a concreate change.
#[derive(Debug)]
pub enum ServerEvent {
    /// Events related to change in the cluster topology
    TopologyChange(TopologyChange),
    /// Events related to change of node status.
    StatusChange(StatusChange),
    /// Events related to schema change.
    SchemaChange(SchemaChange),
}

impl PartialEq<SimpleServerEvent> for ServerEvent {
    fn eq(&self, event: &SimpleServerEvent) -> bool {
        &SimpleServerEvent::from(self) == event
    }
}

impl FromCursor for ServerEvent {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> ServerEvent {
        let event_type = CString::from_cursor(&mut cursor);
        match event_type.as_str() {
            TOPOLOGY_CHANGE => {
                ServerEvent::TopologyChange(TopologyChange::from_cursor(&mut cursor))
            }
            STATUS_CHANGE => ServerEvent::StatusChange(StatusChange::from_cursor(&mut cursor)),
            SCHEMA_CHANGE => ServerEvent::SchemaChange(SchemaChange::from_cursor(&mut cursor)),
            _ => unreachable!(),
        }
    }
}

/// Events related to change in the cluster topology
#[derive(Debug)]
pub struct TopologyChange {
    pub change_type: TopologyChangeType,
    pub addr: CInet,
}

impl FromCursor for TopologyChange {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> TopologyChange {
        let change_type = TopologyChangeType::from_cursor(&mut cursor);
        let addr = CInet::from_cursor(&mut cursor);

        TopologyChange {
            change_type: change_type,
            addr: addr,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TopologyChangeType {
    NewNode,
    RemovedNode,
}

impl FromCursor for TopologyChangeType {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> TopologyChangeType {
        match CString::from_cursor(&mut cursor).as_str() {
            NEW_NODE => TopologyChangeType::NewNode,
            REMOVED_NODE => TopologyChangeType::RemovedNode,
            _ => unreachable!(),
        }
    }
}

/// Events related to change of node status.
#[derive(Debug)]
pub struct StatusChange {
    pub change_type: StatusChangeType,
    pub addr: CInet,
}

impl FromCursor for StatusChange {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> StatusChange {
        let change_type = StatusChangeType::from_cursor(&mut cursor);
        let addr = CInet::from_cursor(&mut cursor);

        StatusChange {
            change_type: change_type,
            addr: addr,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StatusChangeType {
    Up,
    Down,
}

impl FromCursor for StatusChangeType {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> StatusChangeType {
        match CString::from_cursor(&mut cursor).as_str() {
            UP => StatusChangeType::Up,
            DOWN => StatusChangeType::Down,
            _ => unreachable!(),
        }
    }
}

/// Events related to schema change.
#[derive(Debug, PartialEq)]
pub struct SchemaChange {
    pub change_type: ChangeType,
    pub target: Target,
    pub options: ChangeSchemeOptions,
}

impl FromCursor for SchemaChange {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> SchemaChange {
        let change_type = ChangeType::from_cursor(&mut cursor);
        let target = Target::from_cursor(&mut cursor);
        let options = ChangeSchemeOptions::from_cursor_and_target(&mut cursor, &target);

        SchemaChange {
            change_type: change_type,
            target: target,
            options: options,
        }
    }
}

/// Represents type of changes.
#[derive(Debug, PartialEq)]
pub enum ChangeType {
    Created,
    Updated,
    Dropped,
}

impl FromCursor for ChangeType {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> ChangeType {
        match CString::from_cursor(&mut cursor).as_str() {
            CREATED => ChangeType::Created,
            UPDATED => ChangeType::Updated,
            DROPPED => ChangeType::Dropped,
            _ => unreachable!(),
        }
    }
}

/// Refers to a target of changes were made.
#[derive(Debug, PartialEq)]
pub enum Target {
    Keyspace,
    Table,
    Type,
    Function,
    Aggregate,
}

impl FromCursor for Target {
    fn from_cursor(mut cursor: &mut Cursor<&[u8]>) -> Target {
        match CString::from_cursor(&mut cursor).as_str() {
            KEYSPACE => Target::Keyspace,
            TABLE => Target::Table,
            TYPE => Target::Type,
            FUNCTION => Target::Function,
            AGGREGATE => Target::Aggregate,
            _ => unreachable!(),
        }
    }
}

/// Option that contains an information about changes were made.
#[derive(Debug, PartialEq)]
pub enum ChangeSchemeOptions {
    /// Changes related to keyspaces. Contains keyspace name.
    Keyspace(String),
    /// Changes related to tables. Contains keyspace and table names.
    TableType((String, String)),
    /// Changes related to functions and aggregations. Contains:
    /// * keyspace containing the user defined function / aggregate
    /// * the function/aggregate name
    /// * list of strings, one string for each argument type (as CQL type)
    FunctionAggregate((String, String, Vec<String>)),
}

impl ChangeSchemeOptions {
    fn from_cursor_and_target(mut cursor: &mut Cursor<&[u8]>,
                              target: &Target)
                              -> ChangeSchemeOptions {
        match target {
            &Target::Keyspace => ChangeSchemeOptions::from_cursor_keyspace(&mut cursor),
            &Target::Table | &Target::Type => {
                ChangeSchemeOptions::from_cursor_table_type(&mut cursor)
            }
            &Target::Function |
            &Target::Aggregate => ChangeSchemeOptions::from_cursor_function_aggregate(&mut cursor),
        }
    }

    fn from_cursor_keyspace(mut cursor: &mut Cursor<&[u8]>) -> ChangeSchemeOptions {
        ChangeSchemeOptions::Keyspace(CString::from_cursor(&mut cursor).into_plain())
    }

    fn from_cursor_table_type(mut cursor: &mut Cursor<&[u8]>) -> ChangeSchemeOptions {
        let keyspace = CString::from_cursor(&mut cursor).into_plain();
        let name = CString::from_cursor(&mut cursor).into_plain();
        ChangeSchemeOptions::TableType((keyspace, name))
    }

    fn from_cursor_function_aggregate(mut cursor: &mut Cursor<&[u8]>) -> ChangeSchemeOptions {
        let keyspace = CString::from_cursor(&mut cursor).into_plain();
        let name = CString::from_cursor(&mut cursor).into_plain();
        let types = CStringList::from_cursor(&mut cursor).into_plain();
        ChangeSchemeOptions::FunctionAggregate((keyspace, name, types))
    }
}

#[cfg(test)]
mod simple_server_event_test {
    use super::*;

    #[test]
    fn as_string() {
        assert_eq!(SimpleServerEvent::TopologyChange.as_string(),
                   "TOPOLOGY_CHANGE".to_string());
        assert_eq!(SimpleServerEvent::StatusChange.as_string(),
                   "STATUS_CHANGE".to_string());
        assert_eq!(SimpleServerEvent::SchemaChange.as_string(),
                   "SCHEMA_CHANGE".to_string());
    }
}

#[cfg(test)]
mod topology_change_type_test {
    use super::*;
    use std::io::Cursor;
    use FromCursor;

    #[test]
    fn from_cursor() {
        let mut new_node: Cursor<Vec<u8>> = Cursor::new(vec![0, 8, 78, 69, 87, 95, 78, 79, 68, 69]);
        assert_eq!(TopologyChangeType::from_cursor(&mut new_node),
                   TopologyChangeType::NewNode);

        let mut removed_node: Cursor<Vec<u8>> = Cursor::new(vec![0, 12, 82, 69, 77, 79, 86, 69,
                                                                 68, 95, 78, 79, 68, 69]);
        assert_eq!(TopologyChangeType::from_cursor(&mut removed_node),
                   TopologyChangeType::RemovedNode);
    }

    #[test]
    #[should_panic]
    fn from_cursor_wrong() {
        let mut wrong: Cursor<Vec<u8>> = Cursor::new(vec![0, 1, 78]);
        let _ = TopologyChangeType::from_cursor(&mut wrong);
    }
}

#[cfg(test)]
mod status_change_type_test {
    use super::*;
    use std::io::Cursor;
    use FromCursor;

    #[test]
    fn from_cursor() {
        let mut up: Cursor<Vec<u8>> = Cursor::new(vec![0, 2, 85, 80]);
        assert_eq!(StatusChangeType::from_cursor(&mut up), StatusChangeType::Up);

        let mut down: Cursor<Vec<u8>> = Cursor::new(vec![0, 4, 68, 79, 87, 78]);
        assert_eq!(StatusChangeType::from_cursor(&mut down),
                   StatusChangeType::Down);
    }

    #[test]
    #[should_panic]
    fn from_cursor_wrong() {
        let mut wrong: Cursor<Vec<u8>> = Cursor::new(vec![0, 1, 78]);
        let _ = StatusChangeType::from_cursor(&mut wrong);
    }
}

#[cfg(test)]
mod schema_change_type_test {
    use super::*;
    use std::io::Cursor;
    use FromCursor;

    #[test]
    fn from_cursor() {
        let mut created: Cursor<Vec<u8>> = Cursor::new(vec![0, 7, 67, 82, 69, 65, 84, 69, 68]);
        assert_eq!(ChangeType::from_cursor(&mut created), ChangeType::Created);

        let mut updated: Cursor<Vec<u8>> = Cursor::new(vec![0, 7, 85, 80, 68, 65, 84, 69, 68]);
        assert_eq!(ChangeType::from_cursor(&mut updated), ChangeType::Updated);

        let mut dropped: Cursor<Vec<u8>> = Cursor::new(vec![0, 7, 68, 82, 79, 80, 80, 69, 68]);
        assert_eq!(ChangeType::from_cursor(&mut dropped), ChangeType::Dropped);
    }

    #[test]
    #[should_panic]
    fn from_cursor_wrong() {
        let mut wrong: Cursor<Vec<u8>> = Cursor::new(vec![0, 1, 78]);
        let _ = ChangeType::from_cursor(&mut wrong);
    }
}

#[cfg(test)]
mod schema_change_target_test {
    use super::*;
    use std::io::Cursor;
    use FromCursor;

    #[test]
    fn from_cursor() {
        let mut keyspace: Cursor<Vec<u8>> = Cursor::new(vec![0, 8, 75, 69, 89, 83, 80, 65, 67, 69]);
        assert_eq!(Target::from_cursor(&mut keyspace), Target::Keyspace);

        let mut table: Cursor<Vec<u8>> = Cursor::new(vec![0, 5, 84, 65, 66, 76, 69]);
        assert_eq!(Target::from_cursor(&mut table), Target::Table);

        let mut _type: Cursor<Vec<u8>> = Cursor::new(vec![0, 4, 84, 89, 80, 69]);
        assert_eq!(Target::from_cursor(&mut _type), Target::Type);

        let mut function: Cursor<Vec<u8>> = Cursor::new(vec![0, 8, 70, 85, 78, 67, 84, 73, 79, 78]);
        assert_eq!(Target::from_cursor(&mut function), Target::Function);

        let mut aggregate: Cursor<Vec<u8>> = Cursor::new(vec![0, 9, 65, 71, 71, 82, 69, 71, 65,
                                                              84, 69]);
        assert_eq!(Target::from_cursor(&mut aggregate), Target::Aggregate);
    }

    #[test]
    #[should_panic]
    fn from_cursor_wrong() {
        let mut wrong: Cursor<Vec<u8>> = Cursor::new(vec![0, 1, 78]);
        let _ = Target::from_cursor(&mut wrong);
    }
}

#[cfg(test)]
mod server_event {
    use super::*;
    use std::io::Cursor;
    use FromCursor;

    #[test]
    fn topology_change_new_node() {
        let bytes = vec![// topology change
                         0,
                         15,
                         84,
                         79,
                         80,
                         79,
                         76,
                         79,
                         71,
                         89,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // new node
                         0,
                         8,
                         78,
                         69,
                         87,
                         95,
                         78,
                         79,
                         68,
                         69,
                         // 127.0.0.1:1
                         0,
                         4,
                         127,
                         0,
                         0,
                         1,
                         0,
                         0,
                         0,
                         1];
        let mut c: Cursor<Vec<u8>> = Cursor::new(bytes);
        let event = ServerEvent::from_cursor(&mut c);
        match event {
            ServerEvent::TopologyChange(ref tc) => {
                assert_eq!(tc.change_type, TopologyChangeType::NewNode);
                assert_eq!(format!("{:?}", tc.addr.addr), "V4(127.0.0.1:1)");
            }
            _ => panic!("should be topology change event"),
        }
    }

    #[test]
    fn topology_change_removed_node() {
        let bytes = vec![// topology change
                         0,
                         15,
                         84,
                         79,
                         80,
                         79,
                         76,
                         79,
                         71,
                         89,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // removed node
                         0,
                         12,
                         82,
                         69,
                         77,
                         79,
                         86,
                         69,
                         68,
                         95,
                         78,
                         79,
                         68,
                         69,
                         // 127.0.0.1:1
                         0,
                         4,
                         127,
                         0,
                         0,
                         1,
                         0,
                         0,
                         0,
                         1];
        let mut c: Cursor<Vec<u8>> = Cursor::new(bytes);
        let event = ServerEvent::from_cursor(&mut c);
        match event {
            ServerEvent::TopologyChange(ref tc) => {
                assert_eq!(tc.change_type, TopologyChangeType::RemovedNode);
                assert_eq!(format!("{:?}", tc.addr.addr), "V4(127.0.0.1:1)");
            }
            _ => panic!("should be topology change event"),
        }
    }

    #[test]
    fn status_change_up() {
        let bytes = vec![// status change
                         0,
                         13,
                         83,
                         84,
                         65,
                         84,
                         85,
                         83,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // up
                         0,
                         2,
                         85,
                         80,
                         // 127.0.0.1:1
                         0,
                         4,
                         127,
                         0,
                         0,
                         1,
                         0,
                         0,
                         0,
                         1];
        let mut c: Cursor<Vec<u8>> = Cursor::new(bytes);
        let event = ServerEvent::from_cursor(&mut c);
        match event {
            ServerEvent::StatusChange(ref tc) => {
                assert_eq!(tc.change_type, StatusChangeType::Up);
                assert_eq!(format!("{:?}", tc.addr.addr), "V4(127.0.0.1:1)");
            }
            _ => panic!("should be status change up"),
        }
    }

    #[test]
    fn status_change_down() {
        let bytes = vec![// status change
                         0,
                         13,
                         83,
                         84,
                         65,
                         84,
                         85,
                         83,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // down
                         0,
                         4,
                         68,
                         79,
                         87,
                         78,
                         // 127.0.0.1:1
                         0,
                         4,
                         127,
                         0,
                         0,
                         1,
                         0,
                         0,
                         0,
                         1];
        let mut c: Cursor<Vec<u8>> = Cursor::new(bytes);
        let event = ServerEvent::from_cursor(&mut c);
        match event {
            ServerEvent::StatusChange(ref tc) => {
                assert_eq!(tc.change_type, StatusChangeType::Down);
                assert_eq!(format!("{:?}", tc.addr.addr), "V4(127.0.0.1:1)");
            }
            _ => panic!("should be status change down"),
        }
    }

    #[test]
    fn schema_change_created() {
        // keyspace
        let keyspace = vec![// schema change
                            0,
                            13,
                            83,
                            67,
                            72,
                            69,
                            77,
                            65,
                            95,
                            67,
                            72,
                            65,
                            78,
                            71,
                            69,
                            // created
                            0,
                            7,
                            67,
                            82,
                            69,
                            65,
                            84,
                            69,
                            68,
                            // keyspace
                            0,
                            8,
                            75,
                            69,
                            89,
                            83,
                            80,
                            65,
                            67,
                            69,
                            // my_ks
                            0,
                            5,
                            109,
                            121,
                            95,
                            107,
                            115];
        let mut ks: Cursor<Vec<u8>> = Cursor::new(keyspace);
        let ks_event = ServerEvent::from_cursor(&mut ks);
        match ks_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Created);
                assert_eq!(_c.target, Target::Keyspace);
                match _c.options {
                    ChangeSchemeOptions::Keyspace(ref ks) => assert_eq!(ks.as_str(), "my_ks"),
                    _ => panic!("should be keyspace"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // table
        let table = vec![// schema change
                         0,
                         13,
                         83,
                         67,
                         72,
                         69,
                         77,
                         65,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // created
                         0,
                         7,
                         67,
                         82,
                         69,
                         65,
                         84,
                         69,
                         68,
                         // table
                         0,
                         5,
                         84,
                         65,
                         66,
                         76,
                         69,
                         // my_ks
                         0,
                         5,
                         109,
                         121,
                         95,
                         107,
                         115,
                         // my_table
                         0,
                         8,
                         109,
                         121,
                         95,
                         116,
                         97,
                         98,
                         108,
                         101];
        let mut tb: Cursor<Vec<u8>> = Cursor::new(table);
        let tb_event = ServerEvent::from_cursor(&mut tb);
        match tb_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Created);
                assert_eq!(_c.target, Target::Table);
                match _c.options {
                    ChangeSchemeOptions::TableType(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "my_table".to_string()))
                    }
                    _ => panic!("should be table"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // type
        let _type = vec![// schema change
                         0,
                         13,
                         83,
                         67,
                         72,
                         69,
                         77,
                         65,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // created
                         0,
                         7,
                         67,
                         82,
                         69,
                         65,
                         84,
                         69,
                         68,
                         // type
                         0,
                         4,
                         84,
                         89,
                         80,
                         69,
                         // my_ks
                         0,
                         5,
                         109,
                         121,
                         95,
                         107,
                         115,
                         // my_table
                         0,
                         8,
                         109,
                         121,
                         95,
                         116,
                         97,
                         98,
                         108,
                         101];
        let mut tp: Cursor<Vec<u8>> = Cursor::new(_type);
        let tp_event = ServerEvent::from_cursor(&mut tp);
        match tp_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Created);
                assert_eq!(_c.target, Target::Type);
                match _c.options {
                    ChangeSchemeOptions::TableType(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "my_table".to_string()))
                    }
                    _ => panic!("should be type"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // function
        let function = vec![// schema change
                            0,
                            13,
                            83,
                            67,
                            72,
                            69,
                            77,
                            65,
                            95,
                            67,
                            72,
                            65,
                            78,
                            71,
                            69,
                            // created
                            0,
                            7,
                            67,
                            82,
                            69,
                            65,
                            84,
                            69,
                            68,
                            // function
                            0,
                            8,
                            70,
                            85,
                            78,
                            67,
                            84,
                            73,
                            79,
                            78,
                            // my_ks
                            0,
                            5,
                            109,
                            121,
                            95,
                            107,
                            115,
                            // name
                            0,
                            4,
                            110,
                            97,
                            109,
                            101,
                            // empty list of parameters
                            0,
                            0];
        let mut fnct: Cursor<Vec<u8>> = Cursor::new(function);
        let fnct_event = ServerEvent::from_cursor(&mut fnct);
        match fnct_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Created);
                assert_eq!(_c.target, Target::Function);
                match _c.options {
                    ChangeSchemeOptions::FunctionAggregate(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "name".to_string(), vec![]))
                    }
                    _ => panic!("should be function"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // function
        let aggregate = vec![// schema change
                             0,
                             13,
                             83,
                             67,
                             72,
                             69,
                             77,
                             65,
                             95,
                             67,
                             72,
                             65,
                             78,
                             71,
                             69,
                             // created
                             0,
                             7,
                             67,
                             82,
                             69,
                             65,
                             84,
                             69,
                             68,
                             // aggregate
                             0,
                             9,
                             65,
                             71,
                             71,
                             82,
                             69,
                             71,
                             65,
                             84,
                             69,
                             // my_ks
                             0,
                             5,
                             109,
                             121,
                             95,
                             107,
                             115,
                             // name
                             0,
                             4,
                             110,
                             97,
                             109,
                             101,
                             // empty list of parameters
                             0,
                             0];
        let mut aggr: Cursor<Vec<u8>> = Cursor::new(aggregate);
        let aggr_event = ServerEvent::from_cursor(&mut aggr);
        match aggr_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Created);
                assert_eq!(_c.target, Target::Aggregate);
                match _c.options {
                    ChangeSchemeOptions::FunctionAggregate(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "name".to_string(), vec![]))
                    }
                    _ => panic!("should be aggregate"),
                }
            }
            _ => panic!("should be schema change"),
        }
    }

    #[test]
    fn schema_change_updated() {
        // keyspace
        let keyspace = vec![// schema change
                            0,
                            13,
                            83,
                            67,
                            72,
                            69,
                            77,
                            65,
                            95,
                            67,
                            72,
                            65,
                            78,
                            71,
                            69,
                            // updated
                            0,
                            7,
                            85,
                            80,
                            68,
                            65,
                            84,
                            69,
                            68,
                            // keyspace
                            0,
                            8,
                            75,
                            69,
                            89,
                            83,
                            80,
                            65,
                            67,
                            69,
                            // my_ks
                            0,
                            5,
                            109,
                            121,
                            95,
                            107,
                            115];
        let mut ks: Cursor<Vec<u8>> = Cursor::new(keyspace);
        let ks_event = ServerEvent::from_cursor(&mut ks);
        match ks_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Updated);
                assert_eq!(_c.target, Target::Keyspace);
                match _c.options {
                    ChangeSchemeOptions::Keyspace(ref ks) => assert_eq!(ks.as_str(), "my_ks"),
                    _ => panic!("should be keyspace"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // table
        let table = vec![// schema change
                         0,
                         13,
                         83,
                         67,
                         72,
                         69,
                         77,
                         65,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // updated
                         0,
                         7,
                         85,
                         80,
                         68,
                         65,
                         84,
                         69,
                         68,
                         // table
                         0,
                         5,
                         84,
                         65,
                         66,
                         76,
                         69,
                         // my_ks
                         0,
                         5,
                         109,
                         121,
                         95,
                         107,
                         115,
                         // my_table
                         0,
                         8,
                         109,
                         121,
                         95,
                         116,
                         97,
                         98,
                         108,
                         101];
        let mut tb: Cursor<Vec<u8>> = Cursor::new(table);
        let tb_event = ServerEvent::from_cursor(&mut tb);
        match tb_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Updated);
                assert_eq!(_c.target, Target::Table);
                match _c.options {
                    ChangeSchemeOptions::TableType(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "my_table".to_string()))
                    }
                    _ => panic!("should be table"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // type
        let _type = vec![// schema change
                         0,
                         13,
                         83,
                         67,
                         72,
                         69,
                         77,
                         65,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // updated
                         0,
                         7,
                         85,
                         80,
                         68,
                         65,
                         84,
                         69,
                         68,
                         // type
                         0,
                         4,
                         84,
                         89,
                         80,
                         69,
                         // my_ks
                         0,
                         5,
                         109,
                         121,
                         95,
                         107,
                         115,
                         // my_table
                         0,
                         8,
                         109,
                         121,
                         95,
                         116,
                         97,
                         98,
                         108,
                         101];
        let mut tp: Cursor<Vec<u8>> = Cursor::new(_type);
        let tp_event = ServerEvent::from_cursor(&mut tp);
        match tp_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Updated);
                assert_eq!(_c.target, Target::Type);
                match _c.options {
                    ChangeSchemeOptions::TableType(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "my_table".to_string()))
                    }
                    _ => panic!("should be type"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // function
        let function = vec![// schema change
                            0,
                            13,
                            83,
                            67,
                            72,
                            69,
                            77,
                            65,
                            95,
                            67,
                            72,
                            65,
                            78,
                            71,
                            69,
                            // updated
                            0,
                            7,
                            85,
                            80,
                            68,
                            65,
                            84,
                            69,
                            68,
                            // function
                            0,
                            8,
                            70,
                            85,
                            78,
                            67,
                            84,
                            73,
                            79,
                            78,
                            // my_ks
                            0,
                            5,
                            109,
                            121,
                            95,
                            107,
                            115,
                            // name
                            0,
                            4,
                            110,
                            97,
                            109,
                            101,
                            // empty list of parameters
                            0,
                            0];
        let mut fnct: Cursor<Vec<u8>> = Cursor::new(function);
        let fnct_event = ServerEvent::from_cursor(&mut fnct);
        match fnct_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Updated);
                assert_eq!(_c.target, Target::Function);
                match _c.options {
                    ChangeSchemeOptions::FunctionAggregate(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "name".to_string(), vec![]))
                    }
                    _ => panic!("should be function"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // function
        let aggregate = vec![// schema change
                             0,
                             13,
                             83,
                             67,
                             72,
                             69,
                             77,
                             65,
                             95,
                             67,
                             72,
                             65,
                             78,
                             71,
                             69,
                             // updated
                             0,
                             7,
                             85,
                             80,
                             68,
                             65,
                             84,
                             69,
                             68,
                             // aggregate
                             0,
                             9,
                             65,
                             71,
                             71,
                             82,
                             69,
                             71,
                             65,
                             84,
                             69,
                             // my_ks
                             0,
                             5,
                             109,
                             121,
                             95,
                             107,
                             115,
                             // name
                             0,
                             4,
                             110,
                             97,
                             109,
                             101,
                             // empty list of parameters
                             0,
                             0];
        let mut aggr: Cursor<Vec<u8>> = Cursor::new(aggregate);
        let aggr_event = ServerEvent::from_cursor(&mut aggr);
        match aggr_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Updated);
                assert_eq!(_c.target, Target::Aggregate);
                match _c.options {
                    ChangeSchemeOptions::FunctionAggregate(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "name".to_string(), vec![]))
                    }
                    _ => panic!("should be aggregate"),
                }
            }
            _ => panic!("should be schema change"),
        }
    }

    #[test]
    fn schema_change_dropped() {
        // keyspace
        let keyspace = vec![// schema change
                            0,
                            13,
                            83,
                            67,
                            72,
                            69,
                            77,
                            65,
                            95,
                            67,
                            72,
                            65,
                            78,
                            71,
                            69,
                            // dropped
                            0,
                            7,
                            68,
                            82,
                            79,
                            80,
                            80,
                            69,
                            68,
                            // keyspace
                            0,
                            8,
                            75,
                            69,
                            89,
                            83,
                            80,
                            65,
                            67,
                            69,
                            // my_ks
                            0,
                            5,
                            109,
                            121,
                            95,
                            107,
                            115];
        let mut ks: Cursor<Vec<u8>> = Cursor::new(keyspace);
        let ks_event = ServerEvent::from_cursor(&mut ks);
        match ks_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Dropped);
                assert_eq!(_c.target, Target::Keyspace);
                match _c.options {
                    ChangeSchemeOptions::Keyspace(ref ks) => assert_eq!(ks.as_str(), "my_ks"),
                    _ => panic!("should be keyspace"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // table
        let table = vec![// schema change
                         0,
                         13,
                         83,
                         67,
                         72,
                         69,
                         77,
                         65,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // dropped
                         0,
                         7,
                         68,
                         82,
                         79,
                         80,
                         80,
                         69,
                         68,
                         // table
                         0,
                         5,
                         84,
                         65,
                         66,
                         76,
                         69,
                         // my_ks
                         0,
                         5,
                         109,
                         121,
                         95,
                         107,
                         115,
                         // my_table
                         0,
                         8,
                         109,
                         121,
                         95,
                         116,
                         97,
                         98,
                         108,
                         101];
        let mut tb: Cursor<Vec<u8>> = Cursor::new(table);
        let tb_event = ServerEvent::from_cursor(&mut tb);
        match tb_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Dropped);
                assert_eq!(_c.target, Target::Table);
                match _c.options {
                    ChangeSchemeOptions::TableType(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "my_table".to_string()))
                    }
                    _ => panic!("should be table"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // type
        let _type = vec![// schema change
                         0,
                         13,
                         83,
                         67,
                         72,
                         69,
                         77,
                         65,
                         95,
                         67,
                         72,
                         65,
                         78,
                         71,
                         69,
                         // dropped
                         0,
                         7,
                         68,
                         82,
                         79,
                         80,
                         80,
                         69,
                         68,
                         // type
                         0,
                         4,
                         84,
                         89,
                         80,
                         69,
                         // my_ks
                         0,
                         5,
                         109,
                         121,
                         95,
                         107,
                         115,
                         // my_table
                         0,
                         8,
                         109,
                         121,
                         95,
                         116,
                         97,
                         98,
                         108,
                         101];
        let mut tp: Cursor<Vec<u8>> = Cursor::new(_type);
        let tp_event = ServerEvent::from_cursor(&mut tp);
        match tp_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Dropped);
                assert_eq!(_c.target, Target::Type);
                match _c.options {
                    ChangeSchemeOptions::TableType(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "my_table".to_string()))
                    }
                    _ => panic!("should be type"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // function
        let function = vec![// schema change
                            0,
                            13,
                            83,
                            67,
                            72,
                            69,
                            77,
                            65,
                            95,
                            67,
                            72,
                            65,
                            78,
                            71,
                            69,
                            // dropped
                            0,
                            7,
                            68,
                            82,
                            79,
                            80,
                            80,
                            69,
                            68,
                            // function
                            0,
                            8,
                            70,
                            85,
                            78,
                            67,
                            84,
                            73,
                            79,
                            78,
                            // my_ks
                            0,
                            5,
                            109,
                            121,
                            95,
                            107,
                            115,
                            // name
                            0,
                            4,
                            110,
                            97,
                            109,
                            101,
                            // empty list of parameters
                            0,
                            0];
        let mut fnct: Cursor<Vec<u8>> = Cursor::new(function);
        let fnct_event = ServerEvent::from_cursor(&mut fnct);
        match fnct_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Dropped);
                assert_eq!(_c.target, Target::Function);
                match _c.options {
                    ChangeSchemeOptions::FunctionAggregate(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "name".to_string(), vec![]))
                    }
                    _ => panic!("should be function"),
                }
            }
            _ => panic!("should be schema change"),
        }
        // function
        let aggregate = vec![// schema change
                             0,
                             13,
                             83,
                             67,
                             72,
                             69,
                             77,
                             65,
                             95,
                             67,
                             72,
                             65,
                             78,
                             71,
                             69,
                             // dropped
                             0,
                             7,
                             68,
                             82,
                             79,
                             80,
                             80,
                             69,
                             68,
                             // aggregate
                             0,
                             9,
                             65,
                             71,
                             71,
                             82,
                             69,
                             71,
                             65,
                             84,
                             69,
                             // my_ks
                             0,
                             5,
                             109,
                             121,
                             95,
                             107,
                             115,
                             // name
                             0,
                             4,
                             110,
                             97,
                             109,
                             101,
                             // empty list of parameters
                             0,
                             0];
        let mut aggr: Cursor<Vec<u8>> = Cursor::new(aggregate);
        let aggr_event = ServerEvent::from_cursor(&mut aggr);
        match aggr_event {
            ServerEvent::SchemaChange(ref _c) => {
                assert_eq!(_c.change_type, ChangeType::Dropped);
                assert_eq!(_c.target, Target::Aggregate);
                match _c.options {
                    ChangeSchemeOptions::FunctionAggregate(ref tt) => {
                        assert_eq!(tt, &("my_ks".to_string(), "name".to_string(), vec![]))
                    }
                    _ => panic!("should be aggregate"),
                }
            }
            _ => panic!("should be schema change"),
        }
    }
}
