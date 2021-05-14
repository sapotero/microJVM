use byteorder::{ReadBytesExt, BigEndian};
use std::{
    path::Path,
    fs::File,
    io::{self, Read},
};
/*
      MD5 checksum 5bea216f2a8c3eef7e5998435f01a067
      Compiled from "Main.java"
    public class Main
      minor version: 0
      major version: 55
      flags: (0x0021) ACC_PUBLIC, ACC_SUPER
      this_class: #2                          // Main
      super_class: #3                         // java/lang/Object
      interfaces: 0, fields: 0, methods: 2, attributes: 1
    Constant pool:
       #1 = Methodref          #3.#12         // java/lang/Object."<init>":()V
       #2 = Class              #13            // Main
       #3 = Class              #14            // java/lang/Object
       #4 = Utf8               <init>
       #5 = Utf8               ()V
       #6 = Utf8               Code
       #7 = Utf8               LineNumberTable
       #8 = Utf8               add
       #9 = Utf8               (II)I
      #10 = Utf8               SourceFile
      #11 = Utf8               Main.java
      #12 = NameAndType        #4:#5          // "<init>":()V
      #13 = Utf8               Main
      #14 = Utf8               java/lang/Object
    {
      public Main();
        descriptor: ()V
        flags: (0x0001) ACC_PUBLIC
        Code:
          stack=1, locals=1, args_size=1
             0: aload_0
             1: invokespecial #1                  // Method java/lang/Object."<init>":()V
             4: return
          LineNumberTable:
            line 3: 0

      public static int add(int, int);
        descriptor: (II)I
        flags: (0x0009) ACC_PUBLIC, ACC_STATIC
        Code:
          stack=2, locals=2, args_size=2
             0: iload_0
             1: iload_1
             2: iadd
             3: ireturn
          LineNumberTable:
            line 5: 0
    }
*/

// C:\Users\egoko\.jdks\adopt-openjdk-11.0.11\bin\javap.exe -verbose -c ..\..\Main.class

#[derive(Debug)]
enum ConstantPoolInfo {
    /*  1  */
    Utf8Info {
        length: u16,
        bytes: Vec<u8>,
        string: String,
    },
    /*  3  */
    IntegerInfo {
        bytes: u32,
    },
    /*  4  */
    FloatInfo {
        bytes: u32,
    },
    /*  5  */
    LongInfo {
        high_bytes: u64,
        low_bytes: u64,
    },
    /*  6  */
    DoubleInfo {
        high_bytes: u64,
        low_bytes: u64,
    },
    /*  7  */
    ClassInfo {
        name_index: u16
    },
    /*  8  */
    StringInfo {
        string_index: u16
    },
    /*  9  */
    FieldRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /* 10  */
    MethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /* 11  */
    InterfaceMethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /* 12  */
    NameAndTypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    /* 15  */
    MethodHandleInfo {
        reference_kind: u8,
        reference_index: u16,
    },
    /* 16  */
    MethodTypeInfo {
        descriptor_index: u16,
    },
    /* 17  */
    DynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /* 18  */
    InvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /* 19  */
    ModuleInfo {
        name_index: u16,
    },
    /* 20  */
    PackageInfo {
        name_index: u16,
    },
    /*  -  */
    NotImplemented,
}

#[derive(Debug)]
struct ConstantPool {}

impl ConstantPool {
    fn from_reader(mut reader: impl Read, constant_pool_count: u16) -> Vec<ConstantPoolInfo> {
        let mut array = vec![];

        for _ in 1..constant_pool_count {
            let tag = reader.read_u8().unwrap();

            let result = match tag {
                1 => {
                    let length = reader.read_u16::<BigEndian>().unwrap();
                    let mut buffer = vec![0u8; length as usize];
                    reader.read_exact(&mut buffer);

                    ConstantPoolInfo::Utf8Info {
                        length,
                        string: String::from_utf8(buffer.clone()).unwrap(),
                        bytes: buffer,
                    }
                }
                3 => ConstantPoolInfo::IntegerInfo {
                    bytes: reader.read_u32::<BigEndian>().unwrap(),
                },
                4 => ConstantPoolInfo::FloatInfo {
                    bytes: reader.read_u32::<BigEndian>().unwrap(),
                },
                5 => ConstantPoolInfo::LongInfo {
                    high_bytes: reader.read_u64::<BigEndian>().unwrap(),
                    low_bytes: reader.read_u64::<BigEndian>().unwrap(),
                },
                6 => ConstantPoolInfo::DoubleInfo {
                    high_bytes: reader.read_u64::<BigEndian>().unwrap(),
                    low_bytes: reader.read_u64::<BigEndian>().unwrap(),
                },
                7 => ConstantPoolInfo::ClassInfo {
                    name_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                8 => ConstantPoolInfo::StringInfo {
                    string_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                9 => ConstantPoolInfo::FieldRefInfo {
                    class_index: reader.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                10 => ConstantPoolInfo::MethodRefInfo {
                    class_index: reader.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                11 => ConstantPoolInfo::InterfaceMethodRefInfo {
                    class_index: reader.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                12 => ConstantPoolInfo::NameAndTypeInfo {
                    name_index: reader.read_u16::<BigEndian>().unwrap(),
                    descriptor_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                15 => ConstantPoolInfo::MethodHandleInfo {
                    reference_kind: reader.read_u8().unwrap(),
                    reference_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                16 => ConstantPoolInfo::MethodTypeInfo {
                    descriptor_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                17 => ConstantPoolInfo::DynamicInfo {
                    bootstrap_method_attr_index: reader.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                18 => ConstantPoolInfo::InvokeDynamicInfo {
                    bootstrap_method_attr_index: reader.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                19 => ConstantPoolInfo::ModuleInfo {
                    name_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                20 => ConstantPoolInfo::PackageInfo {
                    name_index: reader.read_u16::<BigEndian>().unwrap(),
                },
                _ => ConstantPoolInfo::NotImplemented
            };

            array.push(result);
        }

        array
    }
}

#[derive(Debug)]
struct MethodsInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attribute_info: Vec<AttributeInfo>
}
impl MethodsInfo {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let access_flags = reader.read_u16::<BigEndian>()?;
        let name_index = reader.read_u16::<BigEndian>()?;
        let descriptor_index = reader.read_u16::<BigEndian>()?;
        let attributes_count = reader.read_u16::<BigEndian>()?;

        let mut attribute_info = vec![];
        for _ in 0..attributes_count {
            attribute_info.push(AttributeInfo::from_reader(&mut reader).unwrap());
        }

        Ok(MethodsInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attribute_info
        })
    }
}

#[derive(Debug)]
struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>
}
impl AttributeInfo {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let attribute_name_index = reader.read_u16::<BigEndian>()?;
        let attribute_length = reader.read_u32::<BigEndian>()?;
        let mut info = vec![0u8; attribute_length as usize];
        reader.read_exact(&mut info);

        Ok(AttributeInfo{
            attribute_length,
            attribute_name_index,
            info
        })
    }
}

#[derive(Debug)]
struct Methods {}

impl Methods {
    fn from_reader(mut reader: impl Read, methods_count: u16) -> Vec<MethodsInfo> {
        let mut array = vec![];

        for _ in 0..methods_count {
            array.push(MethodsInfo::from_reader(&mut reader).unwrap());
        }

        array
    }
}
#[derive(Debug)]
struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attribute_info: Vec<AttributeInfo>
}

impl FieldInfo {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let access_flags = reader.read_u16::<BigEndian>()?;
        let name_index = reader.read_u16::<BigEndian>()?;
        let descriptor_index = reader.read_u16::<BigEndian>()?;
        let attributes_count = reader.read_u16::<BigEndian>()?;

        let mut attribute_info = vec![];
        for _ in 0..attributes_count {
            attribute_info.push(AttributeInfo::from_reader(&mut reader).unwrap());
        }

        Ok(FieldInfo{
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attribute_info
        })
    }
}

#[derive(Debug)]
struct Fields {}

impl Fields {
    fn from_reader(mut reader: impl Read, fields_count: u16) -> Vec<FieldInfo> {
        let mut array = vec![];

        for _ in 0..fields_count {
            array.push(FieldInfo::from_reader(&mut reader).unwrap());
        }

        array
    }
}

#[derive(Debug)]
struct Attributes {}

impl Attributes {
    fn from_reader(mut reader: impl Read, attributes_count: u16) -> Vec<AttributeInfo> {
        let mut array = vec![];

        for _ in 0..attributes_count {
            array.push(AttributeInfo::from_reader(&mut reader).unwrap());
        }

        array
    }
}

#[derive(Debug)]
struct Class {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool_info: Vec<ConstantPoolInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u8>,
    fields_count: u16,
    field_info: Vec<FieldInfo>,
    methods_count: u16,
    methods_info: Vec<MethodsInfo>,
    attributes_count: u16,
    attribute_info: Vec<AttributeInfo>
}

impl Class {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let magic = reader.read_u32::<BigEndian>()?;
        let minor_version = reader.read_u16::<BigEndian>()?;
        let major_version = reader.read_u16::<BigEndian>()?;
        let constant_pool_count = reader.read_u16::<BigEndian>()?;
        let constant_pool_info = ConstantPool::from_reader(&mut reader, constant_pool_count);
        let access_flags = reader.read_u16::<BigEndian>()?;
        let this_class = reader.read_u16::<BigEndian>()?;
        let super_class = reader.read_u16::<BigEndian>()?;
        let interfaces_count = reader.read_u16::<BigEndian>()?;

        let mut interfaces = vec![0u8; interfaces_count as usize];
        reader.read_exact(&mut interfaces);

        let fields_count = reader.read_u16::<BigEndian>()?;
        let field_info = Fields::from_reader(&mut reader, fields_count);

        let methods_count = reader.read_u16::<BigEndian>()?;
        let methods_info = Methods::from_reader(&mut reader, methods_count);

        let attributes_count = reader.read_u16::<BigEndian>()?;
        let attribute_info = Attributes::from_reader(&mut reader, attributes_count);

        Ok(Class {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool_info,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            field_info,
            methods_count,
            methods_info,
            attributes_count,
            attribute_info
        })
    }
}

fn main() {
    let path = Path::new("Main.class");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let config = Class::from_reader(file);
    println!("Read structure: {:#?}", config.unwrap());
}
