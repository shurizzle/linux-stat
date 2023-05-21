#![allow(dead_code)]

// This file automatically generate. Do not edit.

use super::Errno;

impl Errno {
    /// No anode
    pub const ENOANO: Self = Self(55);
    /// State not recoverable
    pub const ENOTRECOVERABLE: Self = Self(131);
    /// Operation not possible due to RF-kill
    pub const ERFKILL: Self = Self(132);
    /// No such file or directory
    pub const ENOENT: Self = Self(2);
    /// Resource deadlock would occur
    pub const EDEADLK: Self = Self(35);
    /// Protocol error
    pub const EPROTO: Self = Self(71);
    /// Network is down
    pub const ENETDOWN: Self = Self(100);
    /// Link number out of range
    pub const ELNRNG: Self = Self(48);
    /// Cannot exec a shared library directly
    pub const ELIBEXEC: Self = Self(83);
    /// Restart syscall
    pub const ERESTARTSYS: Self = Self(512);
    /// open found a stale dentry
    pub const EOPENSTALE: Self = Self(518);
    /// Protocol not available
    pub const ENOPROTOOPT: Self = Self(92);
    /// Key has expired
    pub const EKEYEXPIRED: Self = Self(127);
    /// No CSI structure available
    pub const ENOCSI: Self = Self(50);
    /// Request initiated, but will not complete before timeout
    pub const EJUKEBOX: Self = Self(528);
    /// Connection timed out
    pub const ETIMEDOUT: Self = Self(110);
    /// Connection reset by peer
    pub const ECONNRESET: Self = Self(104);
    /// Stale file handle
    pub const ESTALE: Self = Self(116);
    /// No data available
    pub const ENODATA: Self = Self(61);
    /// No medium found
    pub const ENOMEDIUM: Self = Self(123);
    /// Text file busy
    pub const ETXTBSY: Self = Self(26);
    /// Cannot assign requested address
    pub const EADDRNOTAVAIL: Self = Self(99);
    /// Cross-device link
    pub const EXDEV: Self = Self(18);
    /// Invalid slot
    pub const EBADSLT: Self = Self(57);
    /// Machine is not on the network
    pub const ENONET: Self = Self(64);
    /// conflict with recalled state
    pub const ERECALLCONFLICT: Self = Self(530);
    /// Too many symbolic links encountered
    pub const ELOOP: Self = Self(40);
    /// Invalid request descriptor
    pub const EBADR: Self = Self(53);
    /// restart if no handler..
    pub const ERESTARTNOHAND: Self = Self(514);
    /// Buffer or request is too small
    pub const ETOOSMALL: Self = Self(525);
    /// Network is unreachable
    pub const ENETUNREACH: Self = Self(101);
    /// Network dropped connection because of reset
    pub const ENETRESET: Self = Self(102);
    /// Socket type not supported
    pub const ESOCKTNOSUPPORT: Self = Self(94);
    /// Exchange full
    pub const EXFULL: Self = Self(54);
    /// Level 3 reset
    pub const EL3RST: Self = Self(47);
    /// Illegal seek
    pub const ESPIPE: Self = Self(29);
    /// Protocol wrong type for socket
    pub const EPROTOTYPE: Self = Self(91);
    /// Srmount error
    pub const ESRMNT: Self = Self(69);
    /// Bad font file format
    pub const EBFONT: Self = Self(59);
    /// File table overflow
    pub const ENFILE: Self = Self(23);
    /// Invalid exchange
    pub const EBADE: Self = Self(52);
    /// File too large
    pub const EFBIG: Self = Self(27);
    /// Timer expired
    pub const ETIME: Self = Self(62);
    /// RFS specific error
    pub const EDOTDOT: Self = Self(73);
    /// Too many references: cannot splice
    pub const ETOOMANYREFS: Self = Self(109);
    /// Protocol not supported
    pub const EPROTONOSUPPORT: Self = Self(93);
    /// Argument list too long
    pub const E2BIG: Self = Self(7);
    /// Address already in use
    pub const EADDRINUSE: Self = Self(98);
    /// restart by calling sys_restart_syscall
    pub const ERESTART_RESTARTBLOCK: Self = Self(516);
    /// Try again
    pub const EAGAIN: Self = Self(11);
    /// Math argument out of domain of func
    pub const EDOM: Self = Self(33);
    /// iocb queued, will get completion event
    pub const EIOCBQUEUED: Self = Self(529);
    /// Key was rejected by service
    pub const EKEYREJECTED: Self = Self(129);
    /// No XENIX semaphores available
    pub const ENAVAIL: Self = Self(119);
    /// Key has been revoked
    pub const EKEYREVOKED: Self = Self(128);
    /// Channel number out of range
    pub const ECHRNG: Self = Self(44);
    /// .lib section in a.out corrupted
    pub const ELIBSCN: Self = Self(81);
    /// Math result not representable
    pub const ERANGE: Self = Self(34);
    /// Invalid request code
    pub const EBADRQC: Self = Self(56);
    /// Remote address changed
    pub const EREMCHG: Self = Self(78);
    /// Socket operation on non-socket
    pub const ENOTSOCK: Self = Self(88);
    /// No space left on device
    pub const ENOSPC: Self = Self(28);
    /// Protocol family not supported
    pub const EPFNOSUPPORT: Self = Self(96);
    /// Illegal byte sequence
    pub const EILSEQ: Self = Self(84);
    /// No such process
    pub const ESRCH: Self = Self(3);
    /// Owner died
    pub const EOWNERDEAD: Self = Self(130);
    /// Quota exceeded
    pub const EDQUOT: Self = Self(122);
    /// Read-only file system
    pub const EROFS: Self = Self(30);
    /// NFS file lock reclaim refused
    pub const ENOGRACE: Self = Self(531);
    /// Package not installed
    pub const ENOPKG: Self = Self(65);
    /// Attempting to link in too many shared libraries
    pub const ELIBMAX: Self = Self(82);
    /// Object is remote
    pub const EREMOTE: Self = Self(66);
    /// Restart if no interrupt
    pub const ERESTARTNOINTR: Self = Self(513);
    /// Not a XENIX named type file
    pub const ENOTNAM: Self = Self(118);
    /// Is a named type file
    pub const EISNAM: Self = Self(120);
    /// No ioctl command
    pub const ENOIOCTLCMD: Self = Self(515);
    /// Address family not supported by protocol
    pub const EAFNOSUPPORT: Self = Self(97);
    /// No such device or address
    pub const ENXIO: Self = Self(6);
    /// Level 3 halted
    pub const EL3HLT: Self = Self(46);
    /// Destination address required
    pub const EDESTADDRREQ: Self = Self(89);
    /// Value too large for defined data type
    pub const EOVERFLOW: Self = Self(75);
    /// Accessing a corrupted shared library
    pub const ELIBBAD: Self = Self(80);
    /// Software caused connection abort
    pub const ECONNABORTED: Self = Self(103);
    /// Bad address
    pub const EFAULT: Self = Self(14);
    /// Structure needs cleaning
    pub const EUCLEAN: Self = Self(117);
    /// No such device
    pub const ENODEV: Self = Self(19);
    /// Operation not supported on transport endpoint
    pub const EOPNOTSUPP: Self = Self(95);
    /// Multihop attempted
    pub const EMULTIHOP: Self = Self(72);
    /// Type not supported by server
    pub const EBADTYPE: Self = Self(527);
    /// File exists
    pub const EEXIST: Self = Self(17);
    /// Illegal NFS file handle
    pub const EBADHANDLE: Self = Self(521);
    /// Too many open files
    pub const EMFILE: Self = Self(24);
    /// Identifier removed
    pub const EIDRM: Self = Self(43);
    /// Advertise error
    pub const EADV: Self = Self(68);
    /// Too many users
    pub const EUSERS: Self = Self(87);
    /// File name too long
    pub const ENAMETOOLONG: Self = Self(36);
    /// Transport endpoint is not connected
    pub const ENOTCONN: Self = Self(107);
    /// Wrong medium type
    pub const EMEDIUMTYPE: Self = Self(124);
    /// No buffer space available
    pub const ENOBUFS: Self = Self(105);
    /// Cannot send after transport endpoint shutdown
    pub const ESHUTDOWN: Self = Self(108);
    /// Not a data message
    pub const EBADMSG: Self = Self(74);
    /// File descriptor in bad state
    pub const EBADFD: Self = Self(77);
    /// Exec format error
    pub const ENOEXEC: Self = Self(8);
    /// Directory not empty
    pub const ENOTEMPTY: Self = Self(39);
    /// Can not access a needed shared library
    pub const ELIBACC: Self = Self(79);
    /// Operation already in progress
    pub const EALREADY: Self = Self(114);
    /// Too many links
    pub const EMLINK: Self = Self(31);
    /// Connection refused
    pub const ECONNREFUSED: Self = Self(111);
    /// Operation not permitted
    pub const EPERM: Self = Self(1);
    /// Streams pipe error
    pub const ESTRPIPE: Self = Self(86);
    /// Cookie is stale
    pub const EBADCOOKIE: Self = Self(523);
    /// Memory page has hardware error
    pub const EHWPOISON: Self = Self(133);
    /// No record locks available
    pub const ENOLCK: Self = Self(37);
    /// Bad file number
    pub const EBADF: Self = Self(9);
    /// Not a directory
    pub const ENOTDIR: Self = Self(20);
    /// Interrupted system call should be restarted
    pub const ERESTART: Self = Self(85);
    /// Not a typewriter
    pub const ENOTTY: Self = Self(25);
    /// No route to host
    pub const EHOSTUNREACH: Self = Self(113);
    /// Host is down
    pub const EHOSTDOWN: Self = Self(112);
    /// Protocol driver not attached
    pub const EUNATCH: Self = Self(49);
    /// Device or resource busy
    pub const EBUSY: Self = Self(16);
    /// Device not a stream
    pub const ENOSTR: Self = Self(60);
    /// Interrupted system call
    pub const EINTR: Self = Self(4);
    /// Operation is not supported
    pub const ENOTSUPP: Self = Self(524);
    /// Level 2 halted
    pub const EL2HLT: Self = Self(51);
    /// Is a directory
    pub const EISDIR: Self = Self(21);
    /// Link has been severed
    pub const ENOLINK: Self = Self(67);
    /// Operation Canceled
    pub const ECANCELED: Self = Self(125);
    /// An untranslatable error occurred
    pub const ESERVERFAULT: Self = Self(526);
    /// Invalid system call number
    pub const ENOSYS: Self = Self(38);
    /// Out of memory
    pub const ENOMEM: Self = Self(12);
    /// Name not unique on network
    pub const ENOTUNIQ: Self = Self(76);
    /// Message too long
    pub const EMSGSIZE: Self = Self(90);
    /// Transport endpoint is already connected
    pub const EISCONN: Self = Self(106);
    /// Parameter not supported
    pub const ENOPARAM: Self = Self(519);
    /// Remote I/O error
    pub const EREMOTEIO: Self = Self(121);
    /// I/O error
    pub const EIO: Self = Self(5);
    /// Update synchronization mismatch
    pub const ENOTSYNC: Self = Self(522);
    /// Operation now in progress
    pub const EINPROGRESS: Self = Self(115);
    /// Required key not available
    pub const ENOKEY: Self = Self(126);
    /// Invalid argument
    pub const EINVAL: Self = Self(22);
    /// Driver requests probe retry
    pub const EPROBE_DEFER: Self = Self(517);
    /// Block device required
    pub const ENOTBLK: Self = Self(15);
    /// Out of streams resources
    pub const ENOSR: Self = Self(63);
    /// Communication error on send
    pub const ECOMM: Self = Self(70);
    /// Broken pipe
    pub const EPIPE: Self = Self(32);
    /// No child processes
    pub const ECHILD: Self = Self(10);
    /// No message of desired type
    pub const ENOMSG: Self = Self(42);
    /// Level 2 not synchronized
    pub const EL2NSYNC: Self = Self(45);
    /// Permission denied
    pub const EACCES: Self = Self(13);
    pub const EWOULDBLOCK: Self = Self::EAGAIN;

    pub(crate) fn name_and_description(&self) -> Option<(&'static str, &'static str)> {
        match self.0 {
            55 => Some(("ENOANO", "No anode")),
            131 => Some(("ENOTRECOVERABLE", "State not recoverable")),
            132 => Some(("ERFKILL", "Operation not possible due to RF-kill")),
            2 => Some(("ENOENT", "No such file or directory")),
            35 => Some(("EDEADLK", "Resource deadlock would occur")),
            71 => Some(("EPROTO", "Protocol error")),
            100 => Some(("ENETDOWN", "Network is down")),
            48 => Some(("ELNRNG", "Link number out of range")),
            83 => Some(("ELIBEXEC", "Cannot exec a shared library directly")),
            512 => Some(("ERESTARTSYS", "Restart syscall")),
            518 => Some(("EOPENSTALE", "open found a stale dentry")),
            92 => Some(("ENOPROTOOPT", "Protocol not available")),
            127 => Some(("EKEYEXPIRED", "Key has expired")),
            50 => Some(("ENOCSI", "No CSI structure available")),
            528 => Some((
                "EJUKEBOX",
                "Request initiated, but will not complete before timeout",
            )),
            110 => Some(("ETIMEDOUT", "Connection timed out")),
            104 => Some(("ECONNRESET", "Connection reset by peer")),
            116 => Some(("ESTALE", "Stale file handle")),
            61 => Some(("ENODATA", "No data available")),
            123 => Some(("ENOMEDIUM", "No medium found")),
            26 => Some(("ETXTBSY", "Text file busy")),
            99 => Some(("EADDRNOTAVAIL", "Cannot assign requested address")),
            18 => Some(("EXDEV", "Cross-device link")),
            57 => Some(("EBADSLT", "Invalid slot")),
            64 => Some(("ENONET", "Machine is not on the network")),
            530 => Some(("ERECALLCONFLICT", "conflict with recalled state")),
            40 => Some(("ELOOP", "Too many symbolic links encountered")),
            53 => Some(("EBADR", "Invalid request descriptor")),
            514 => Some(("ERESTARTNOHAND", "restart if no handler..")),
            525 => Some(("ETOOSMALL", "Buffer or request is too small")),
            101 => Some(("ENETUNREACH", "Network is unreachable")),
            102 => Some(("ENETRESET", "Network dropped connection because of reset")),
            94 => Some(("ESOCKTNOSUPPORT", "Socket type not supported")),
            54 => Some(("EXFULL", "Exchange full")),
            47 => Some(("EL3RST", "Level 3 reset")),
            29 => Some(("ESPIPE", "Illegal seek")),
            91 => Some(("EPROTOTYPE", "Protocol wrong type for socket")),
            69 => Some(("ESRMNT", "Srmount error")),
            59 => Some(("EBFONT", "Bad font file format")),
            23 => Some(("ENFILE", "File table overflow")),
            52 => Some(("EBADE", "Invalid exchange")),
            27 => Some(("EFBIG", "File too large")),
            62 => Some(("ETIME", "Timer expired")),
            73 => Some(("EDOTDOT", "RFS specific error")),
            109 => Some(("ETOOMANYREFS", "Too many references: cannot splice")),
            93 => Some(("EPROTONOSUPPORT", "Protocol not supported")),
            7 => Some(("E2BIG", "Argument list too long")),
            98 => Some(("EADDRINUSE", "Address already in use")),
            516 => Some((
                "ERESTART_RESTARTBLOCK",
                "restart by calling sys_restart_syscall",
            )),
            11 => Some(("EAGAIN", "Try again")),
            33 => Some(("EDOM", "Math argument out of domain of func")),
            529 => Some(("EIOCBQUEUED", "iocb queued, will get completion event")),
            129 => Some(("EKEYREJECTED", "Key was rejected by service")),
            119 => Some(("ENAVAIL", "No XENIX semaphores available")),
            128 => Some(("EKEYREVOKED", "Key has been revoked")),
            44 => Some(("ECHRNG", "Channel number out of range")),
            81 => Some(("ELIBSCN", ".lib section in a.out corrupted")),
            34 => Some(("ERANGE", "Math result not representable")),
            56 => Some(("EBADRQC", "Invalid request code")),
            78 => Some(("EREMCHG", "Remote address changed")),
            88 => Some(("ENOTSOCK", "Socket operation on non-socket")),
            28 => Some(("ENOSPC", "No space left on device")),
            96 => Some(("EPFNOSUPPORT", "Protocol family not supported")),
            84 => Some(("EILSEQ", "Illegal byte sequence")),
            3 => Some(("ESRCH", "No such process")),
            130 => Some(("EOWNERDEAD", "Owner died")),
            122 => Some(("EDQUOT", "Quota exceeded")),
            30 => Some(("EROFS", "Read-only file system")),
            531 => Some(("ENOGRACE", "NFS file lock reclaim refused")),
            65 => Some(("ENOPKG", "Package not installed")),
            82 => Some(("ELIBMAX", "Attempting to link in too many shared libraries")),
            66 => Some(("EREMOTE", "Object is remote")),
            513 => Some(("ERESTARTNOINTR", "Restart if no interrupt")),
            118 => Some(("ENOTNAM", "Not a XENIX named type file")),
            120 => Some(("EISNAM", "Is a named type file")),
            515 => Some(("ENOIOCTLCMD", "No ioctl command")),
            97 => Some(("EAFNOSUPPORT", "Address family not supported by protocol")),
            6 => Some(("ENXIO", "No such device or address")),
            46 => Some(("EL3HLT", "Level 3 halted")),
            89 => Some(("EDESTADDRREQ", "Destination address required")),
            75 => Some(("EOVERFLOW", "Value too large for defined data type")),
            80 => Some(("ELIBBAD", "Accessing a corrupted shared library")),
            103 => Some(("ECONNABORTED", "Software caused connection abort")),
            14 => Some(("EFAULT", "Bad address")),
            117 => Some(("EUCLEAN", "Structure needs cleaning")),
            19 => Some(("ENODEV", "No such device")),
            95 => Some((
                "EOPNOTSUPP",
                "Operation not supported on transport endpoint",
            )),
            72 => Some(("EMULTIHOP", "Multihop attempted")),
            527 => Some(("EBADTYPE", "Type not supported by server")),
            17 => Some(("EEXIST", "File exists")),
            521 => Some(("EBADHANDLE", "Illegal NFS file handle")),
            24 => Some(("EMFILE", "Too many open files")),
            43 => Some(("EIDRM", "Identifier removed")),
            68 => Some(("EADV", "Advertise error")),
            87 => Some(("EUSERS", "Too many users")),
            36 => Some(("ENAMETOOLONG", "File name too long")),
            107 => Some(("ENOTCONN", "Transport endpoint is not connected")),
            124 => Some(("EMEDIUMTYPE", "Wrong medium type")),
            105 => Some(("ENOBUFS", "No buffer space available")),
            108 => Some(("ESHUTDOWN", "Cannot send after transport endpoint shutdown")),
            74 => Some(("EBADMSG", "Not a data message")),
            77 => Some(("EBADFD", "File descriptor in bad state")),
            8 => Some(("ENOEXEC", "Exec format error")),
            39 => Some(("ENOTEMPTY", "Directory not empty")),
            79 => Some(("ELIBACC", "Can not access a needed shared library")),
            114 => Some(("EALREADY", "Operation already in progress")),
            31 => Some(("EMLINK", "Too many links")),
            111 => Some(("ECONNREFUSED", "Connection refused")),
            1 => Some(("EPERM", "Operation not permitted")),
            86 => Some(("ESTRPIPE", "Streams pipe error")),
            523 => Some(("EBADCOOKIE", "Cookie is stale")),
            133 => Some(("EHWPOISON", "Memory page has hardware error")),
            37 => Some(("ENOLCK", "No record locks available")),
            9 => Some(("EBADF", "Bad file number")),
            20 => Some(("ENOTDIR", "Not a directory")),
            85 => Some(("ERESTART", "Interrupted system call should be restarted")),
            25 => Some(("ENOTTY", "Not a typewriter")),
            113 => Some(("EHOSTUNREACH", "No route to host")),
            112 => Some(("EHOSTDOWN", "Host is down")),
            49 => Some(("EUNATCH", "Protocol driver not attached")),
            16 => Some(("EBUSY", "Device or resource busy")),
            60 => Some(("ENOSTR", "Device not a stream")),
            4 => Some(("EINTR", "Interrupted system call")),
            524 => Some(("ENOTSUPP", "Operation is not supported")),
            51 => Some(("EL2HLT", "Level 2 halted")),
            21 => Some(("EISDIR", "Is a directory")),
            67 => Some(("ENOLINK", "Link has been severed")),
            125 => Some(("ECANCELED", "Operation Canceled")),
            526 => Some(("ESERVERFAULT", "An untranslatable error occurred")),
            38 => Some(("ENOSYS", "Invalid system call number")),
            12 => Some(("ENOMEM", "Out of memory")),
            76 => Some(("ENOTUNIQ", "Name not unique on network")),
            90 => Some(("EMSGSIZE", "Message too long")),
            106 => Some(("EISCONN", "Transport endpoint is already connected")),
            519 => Some(("ENOPARAM", "Parameter not supported")),
            121 => Some(("EREMOTEIO", "Remote I/O error")),
            5 => Some(("EIO", "I/O error")),
            522 => Some(("ENOTSYNC", "Update synchronization mismatch")),
            115 => Some(("EINPROGRESS", "Operation now in progress")),
            126 => Some(("ENOKEY", "Required key not available")),
            22 => Some(("EINVAL", "Invalid argument")),
            517 => Some(("EPROBE_DEFER", "Driver requests probe retry")),
            15 => Some(("ENOTBLK", "Block device required")),
            63 => Some(("ENOSR", "Out of streams resources")),
            70 => Some(("ECOMM", "Communication error on send")),
            32 => Some(("EPIPE", "Broken pipe")),
            10 => Some(("ECHILD", "No child processes")),
            42 => Some(("ENOMSG", "No message of desired type")),
            45 => Some(("EL2NSYNC", "Level 2 not synchronized")),
            13 => Some(("EACCES", "Permission denied")),
            _ => None,
        }
    }
}
