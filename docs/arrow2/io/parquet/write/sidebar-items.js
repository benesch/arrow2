initSidebarItems({"enum":[["CompressedPage","A [`CompressedPage`] is a compressed, encoded representation of a Parquet page. It holds actual data and thus cloning it is expensive."],["Compression",""],["Encoding",""],["ParquetType","Representation of a Parquet type. Used to describe primitive leaf fields and structs, including top-level schema. Note that the top-level schema type is represented using `GroupType` whose repetition is `None`."],["Version",""]],"fn":[["array_to_page",""],["array_to_pages","Returns an iterator of compressed pages,"],["can_encode","Checks whether the `data_type` can be encoded as `encoding`. Note that this is whether this implementation supports it, which is a subset of what the parquet spec allows."],["parquet_write_file",""],["to_parquet_schema","Creates a parquet [`SchemaDescriptor`] from a [`Schema`]."],["to_parquet_type",""],["write_file","Writes"]],"mod":[["stream",""]],"struct":[["ColumnDescriptor","A descriptor for leaf-level primitive columns. This encapsulates information such as definition and repetition levels and is used to re-assemble nested data."],["CompressedDataPage","A [`CompressedDataPage`] is compressed, encoded representation of a Parquet data page. It holds actual data and thus cloning it is expensive."],["DynIter","[`DynIter`] is an implementation of a single-threaded, dynamically-typed iterator."],["RowGroupIterator","An iterator adapter that converts an iterator over [`RecordBatch`] into an iterator of row groups. Use it to create an iterator consumable by the parquet’s API."],["SchemaDescriptor","A schema descriptor. This encapsulates the top-level schemas for all the columns, as well as all descriptors for all the primitive columns."],["WriteOptions",""]],"type":[["KeyValue",""],["RowGroupIter",""]]});