//! Various block state values.
//! See the `libcraft-blocks` crate
//! for actual block definitions.

use serde::{Deserialize, Serialize};
use std::str::FromStr;


macro_rules! impl_enum {
    {
        $(  
            $(#[$doc:meta])*
            $name:ident {
                $($(#[$vdoc:meta])* $variants:ident),* $(,)?
            }
        )*
    } => {
        $(
            $(#[$doc])*
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
            pub enum $name {
                $($(#[$vdoc])* $variants),*
            }
            impl FromStr for $name {
                type Err = ();
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    #[allow(non_snake_case)]
                    mod strings {
                        $(
                            pub mod $variants {
                                const PASCAL_CASE: &[u8] = stringify!($variants).as_bytes();
                                const LEN: usize = {
                                    let mut len = PASCAL_CASE.len();
                                    let mut i = 1;
                                    while i < PASCAL_CASE.len() {
                                        if PASCAL_CASE[i].is_ascii_uppercase() {
                                            len += 1;
                                        }
                                        i += 1;
                                    }
                                    len
                                };
                                pub const SNAKE_CASE: &[u8] = &{
                                    let mut buffer = [0; LEN];
                                    buffer[0] = PASCAL_CASE[0].to_ascii_lowercase();
                                    let mut i = 1;
                                    let mut j = 1;
                                    while i < buffer.len() {
                                        let c = PASCAL_CASE[j];
                                        if c.is_ascii_uppercase() {
                                            buffer[i] = b'_';
                                            i += 1;
                                            buffer[i] = c.to_ascii_lowercase();
                                        } else {
                                            buffer[i] = c;
                                        }
                                        i += 1;
                                        j += 1;
                                    }
                                    buffer
                                };
                            }
                        )*
                    }
                    match s.as_bytes() {
                        $(strings::$variants::SNAKE_CASE => Ok(Self::$variants),)*
                        _ => Err(())
                    }
                }
            }
        )*
    };
}



impl_enum! {
    /// Direction a block is facing in.
    #[repr(u8)]
    BlockFace {
        South,
        SouthSouthwest,
        Southwest,
        WestSouthwest,
        West,
        WestNorthwest,
        Northwest,
        NorthNorthwest,
        North,
        NorthNortheast,
        Northeast,
        EastNortheast,
        East,
        EastSoutheast,
        Southeast,
        SouthSoutheast,
    }

    /// Size of bamboo leaves.
    BambooLeaves {
        None,
        Small,
        Large,
    }
    
    /// Part of a bed.
    BedPart {
        Foot,
        Head,
    }
    
    /// How a bell is attached.
    BellAttachment {
        Ceiling,
        Floor,
        SingleWall,
        DoubleWall,
    }
    
    /// An axis. Used for bone blocks, portal blocks, chains, etc.
    Axis {
        X,
        Y,
        Z,
    }
    
    /// Block face a button or grindstone is attached to.
    AttachedFace {
        Ceiling,
        Floor,
        Wall,
    }
    
    /// Type of a chest.
    ChestType {
        Single,
        /// Double chest; this block is on the left side.
        Left,
        /// Double chest; this block is on the right side.
        Right,
    }
    /// Which half of a door or flower block is.
    BlockHalf {
        Lower,
        Upper,
    }
    
    /// Which half of stairs.
    StairHalf {
        Bottom,
        Top,
    }
    
    /// To which side a door's hinge is.
    DoorHinge {
        Left,
        Right,
    }
    
    /// Orientation of a jigsaw block.
    Orientation {
        DownEast,
        DownNorth,
        DownSouth,
        DownWest,
        EastUp,
        NorthUp,
        SouthUp,
        UpEast,
        UpNorth,
        UpSouth,
        UpWest,
        WestUp,
    }
    
    /// A note block instrument.
    Instrument {
        Banjo,
        Basedrum,
        Bass,
        Bell,
        Bit,
        Chime,
        CowBell,
        Didgeridoo,
        Flute,
        Guitar,
        Harp,
        Hat,
        IronXylophone,
        Pling,
        Snare,
        Xylophone,
    }
    
    /// Type of a slab block.
    SlabType {
        Bottom,
        Top,
        Double,
    }
    
    /// Type of a moving piston or piston head.
    PistonType {
        Normal,
        Sticky,
    }
    
    /// Shape of a rail block.
    RailShape {
        EastWest,
        NorthEast,
        NorthSouth,
        NorthWest,
        SouthEast,
        SouthWest,
        AscendingEast,
        AscendingNorth,
        AscendingSouth,
        AscendingWest,
    }

    /// Mode of a redstone comparator.
    ComparatorMode {
        Compare,
        Subtract,
    }

    /// How a redstone dust connects to a given side.
    RedstoneConnection {
        None,
        Side,
        Up,
    }

    /// Shape of a stairs block.
    StairShape {
        InnerLeft,
        InnerRight,
        OuterLeft,
        OuterRight,
        Straight,
    }
    
    /// Mode of a structure block.
    StructureBlockMode {
        Corner,
        Data,
        Load,
        Save,
    }
    /// How a wall connects to a given direction.
    WallConnection {
        None,
        Low,
        Tall,
    }
}

#[test]
fn parses() {
    assert_eq!("low".parse(), Ok(WallConnection::Low));
}
