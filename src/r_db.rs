use rocksdb::{DBWithThreadMode, SingleThreaded};

lazy_static! {
    pub static ref RDB: DBWithThreadMode<SingleThreaded> = {
        let path = "_path_for_rocksdb_storage";
        rocksdb::DB::open_default(path).unwrap()
    };
}
