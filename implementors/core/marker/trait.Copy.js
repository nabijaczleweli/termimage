(function() {var implementors = {};
implementors["adler"] = [{"text":"impl Copy for Adler32","synthetic":false,"types":[]}];
implementors["aho_corasick"] = [{"text":"impl Copy for MatchKind","synthetic":false,"types":[]},{"text":"impl Copy for MatchKind","synthetic":false,"types":[]}];
implementors["ansi_term"] = [{"text":"impl Copy for Prefix","synthetic":false,"types":[]},{"text":"impl Copy for Infix","synthetic":false,"types":[]},{"text":"impl Copy for Suffix","synthetic":false,"types":[]},{"text":"impl Copy for Style","synthetic":false,"types":[]},{"text":"impl Copy for Colour","synthetic":false,"types":[]}];
implementors["atty"] = [{"text":"impl Copy for Stream","synthetic":false,"types":[]}];
implementors["bytemuck"] = [{"text":"impl Copy for PodCastError","synthetic":false,"types":[]}];
implementors["byteorder"] = [{"text":"impl Copy for BigEndian","synthetic":false,"types":[]},{"text":"impl Copy for LittleEndian","synthetic":false,"types":[]}];
implementors["clap"] = [{"text":"impl Copy for AppSettings","synthetic":false,"types":[]},{"text":"impl Copy for ArgSettings","synthetic":false,"types":[]},{"text":"impl Copy for Shell","synthetic":false,"types":[]},{"text":"impl Copy for ErrorKind","synthetic":false,"types":[]}];
implementors["crossbeam_channel"] = [{"text":"impl&lt;T:&nbsp;Copy&gt; Copy for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy&gt; Copy for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy&gt; Copy for SendTimeoutError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Copy for RecvError","synthetic":false,"types":[]},{"text":"impl Copy for TryRecvError","synthetic":false,"types":[]},{"text":"impl Copy for RecvTimeoutError","synthetic":false,"types":[]},{"text":"impl Copy for TrySelectError","synthetic":false,"types":[]},{"text":"impl Copy for SelectTimeoutError","synthetic":false,"types":[]},{"text":"impl Copy for TryReadyError","synthetic":false,"types":[]},{"text":"impl Copy for ReadyTimeoutError","synthetic":false,"types":[]}];
implementors["crossbeam_deque"] = [{"text":"impl&lt;T:&nbsp;Copy&gt; Copy for Steal&lt;T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?Sized + Pointable, '_&gt; Copy for Shared&lt;'_, T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_utils"] = [{"text":"impl&lt;T:&nbsp;Copy&gt; Copy for CachePadded&lt;T&gt;","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl Copy for Compression","synthetic":false,"types":[]},{"text":"impl Copy for SpecialOptions","synthetic":false,"types":[]},{"text":"impl Copy for CompressionOptions","synthetic":false,"types":[]},{"text":"impl Copy for MatchingType","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L:&nbsp;Copy, R:&nbsp;Copy&gt; Copy for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["gif"] = [{"text":"impl Copy for DisposalMethod","synthetic":false,"types":[]},{"text":"impl Copy for Block","synthetic":false,"types":[]},{"text":"impl Copy for AnyExtension","synthetic":false,"types":[]},{"text":"impl Copy for Extension","synthetic":false,"types":[]},{"text":"impl Copy for ColorOutput","synthetic":false,"types":[]}];
implementors["image"] = [{"text":"impl Copy for Rect","synthetic":false,"types":[]},{"text":"impl Copy for BiLevel","synthetic":false,"types":[]},{"text":"impl Copy for FilterType","synthetic":false,"types":[]},{"text":"impl Copy for SampleLayout","synthetic":false,"types":[]},{"text":"impl Copy for Error","synthetic":false,"types":[]},{"text":"impl Copy for NormalForm","synthetic":false,"types":[]},{"text":"impl Copy for DXTVariant","synthetic":false,"types":[]},{"text":"impl Copy for Rgbe8Pixel","synthetic":false,"types":[]},{"text":"impl Copy for PixelDensityUnit","synthetic":false,"types":[]},{"text":"impl Copy for PixelDensity","synthetic":false,"types":[]},{"text":"impl Copy for CompressionType","synthetic":false,"types":[]},{"text":"impl Copy for FilterType","synthetic":false,"types":[]},{"text":"impl Copy for SampleEncoding","synthetic":false,"types":[]},{"text":"impl Copy for PNMSubtype","synthetic":false,"types":[]},{"text":"impl Copy for BitmapHeader","synthetic":false,"types":[]},{"text":"impl Copy for GraymapHeader","synthetic":false,"types":[]},{"text":"impl Copy for PixmapHeader","synthetic":false,"types":[]},{"text":"impl Copy for Delay","synthetic":false,"types":[]},{"text":"impl Copy for ColorType","synthetic":false,"types":[]},{"text":"impl Copy for ExtendedColorType","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + Primitive&gt; Copy for Rgb&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + Primitive&gt; Copy for Bgr&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + Primitive&gt; Copy for Luma&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + Primitive&gt; Copy for Rgba&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + Primitive&gt; Copy for Bgra&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy + Primitive&gt; Copy for LumaA&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Copy for ImageFormat","synthetic":false,"types":[]},{"text":"impl Copy for Progress","synthetic":false,"types":[]}];
implementors["jpeg_decoder"] = [{"text":"impl Copy for PixelFormat","synthetic":false,"types":[]},{"text":"impl Copy for ImageInfo","synthetic":false,"types":[]}];
implementors["libc"] = [{"text":"impl Copy for DIR","synthetic":false,"types":[]},{"text":"impl Copy for group","synthetic":false,"types":[]},{"text":"impl Copy for utimbuf","synthetic":false,"types":[]},{"text":"impl Copy for timeval","synthetic":false,"types":[]},{"text":"impl Copy for timespec","synthetic":false,"types":[]},{"text":"impl Copy for rlimit","synthetic":false,"types":[]},{"text":"impl Copy for rusage","synthetic":false,"types":[]},{"text":"impl Copy for ipv6_mreq","synthetic":false,"types":[]},{"text":"impl Copy for hostent","synthetic":false,"types":[]},{"text":"impl Copy for iovec","synthetic":false,"types":[]},{"text":"impl Copy for pollfd","synthetic":false,"types":[]},{"text":"impl Copy for winsize","synthetic":false,"types":[]},{"text":"impl Copy for linger","synthetic":false,"types":[]},{"text":"impl Copy for sigval","synthetic":false,"types":[]},{"text":"impl Copy for itimerval","synthetic":false,"types":[]},{"text":"impl Copy for tms","synthetic":false,"types":[]},{"text":"impl Copy for servent","synthetic":false,"types":[]},{"text":"impl Copy for protoent","synthetic":false,"types":[]},{"text":"impl Copy for FILE","synthetic":false,"types":[]},{"text":"impl Copy for fpos_t","synthetic":false,"types":[]},{"text":"impl Copy for timezone","synthetic":false,"types":[]},{"text":"impl Copy for in_addr","synthetic":false,"types":[]},{"text":"impl Copy for ip_mreq","synthetic":false,"types":[]},{"text":"impl Copy for ip_mreq_source","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_in","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_in6","synthetic":false,"types":[]},{"text":"impl Copy for addrinfo","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_ll","synthetic":false,"types":[]},{"text":"impl Copy for fd_set","synthetic":false,"types":[]},{"text":"impl Copy for tm","synthetic":false,"types":[]},{"text":"impl Copy for sched_param","synthetic":false,"types":[]},{"text":"impl Copy for Dl_info","synthetic":false,"types":[]},{"text":"impl Copy for lconv","synthetic":false,"types":[]},{"text":"impl Copy for in_pktinfo","synthetic":false,"types":[]},{"text":"impl Copy for ifaddrs","synthetic":false,"types":[]},{"text":"impl Copy for in6_rtmsg","synthetic":false,"types":[]},{"text":"impl Copy for arpreq","synthetic":false,"types":[]},{"text":"impl Copy for arpreq_old","synthetic":false,"types":[]},{"text":"impl Copy for arphdr","synthetic":false,"types":[]},{"text":"impl Copy for mmsghdr","synthetic":false,"types":[]},{"text":"impl Copy for epoll_event","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_un","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_storage","synthetic":false,"types":[]},{"text":"impl Copy for utsname","synthetic":false,"types":[]},{"text":"impl Copy for sigevent","synthetic":false,"types":[]},{"text":"impl Copy for fpos64_t","synthetic":false,"types":[]},{"text":"impl Copy for rlimit64","synthetic":false,"types":[]},{"text":"impl Copy for glob_t","synthetic":false,"types":[]},{"text":"impl Copy for passwd","synthetic":false,"types":[]},{"text":"impl Copy for spwd","synthetic":false,"types":[]},{"text":"impl Copy for dqblk","synthetic":false,"types":[]},{"text":"impl Copy for signalfd_siginfo","synthetic":false,"types":[]},{"text":"impl Copy for itimerspec","synthetic":false,"types":[]},{"text":"impl Copy for fsid_t","synthetic":false,"types":[]},{"text":"impl Copy for packet_mreq","synthetic":false,"types":[]},{"text":"impl Copy for cpu_set_t","synthetic":false,"types":[]},{"text":"impl Copy for if_nameindex","synthetic":false,"types":[]},{"text":"impl Copy for msginfo","synthetic":false,"types":[]},{"text":"impl Copy for sembuf","synthetic":false,"types":[]},{"text":"impl Copy for input_event","synthetic":false,"types":[]},{"text":"impl Copy for input_id","synthetic":false,"types":[]},{"text":"impl Copy for input_absinfo","synthetic":false,"types":[]},{"text":"impl Copy for input_keymap_entry","synthetic":false,"types":[]},{"text":"impl Copy for input_mask","synthetic":false,"types":[]},{"text":"impl Copy for ff_replay","synthetic":false,"types":[]},{"text":"impl Copy for ff_trigger","synthetic":false,"types":[]},{"text":"impl Copy for ff_envelope","synthetic":false,"types":[]},{"text":"impl Copy for ff_constant_effect","synthetic":false,"types":[]},{"text":"impl Copy for ff_ramp_effect","synthetic":false,"types":[]},{"text":"impl Copy for ff_condition_effect","synthetic":false,"types":[]},{"text":"impl Copy for ff_periodic_effect","synthetic":false,"types":[]},{"text":"impl Copy for ff_rumble_effect","synthetic":false,"types":[]},{"text":"impl Copy for ff_effect","synthetic":false,"types":[]},{"text":"impl Copy for dl_phdr_info","synthetic":false,"types":[]},{"text":"impl Copy for Elf32_Ehdr","synthetic":false,"types":[]},{"text":"impl Copy for Elf64_Ehdr","synthetic":false,"types":[]},{"text":"impl Copy for Elf32_Sym","synthetic":false,"types":[]},{"text":"impl Copy for Elf64_Sym","synthetic":false,"types":[]},{"text":"impl Copy for Elf32_Phdr","synthetic":false,"types":[]},{"text":"impl Copy for Elf64_Phdr","synthetic":false,"types":[]},{"text":"impl Copy for Elf32_Shdr","synthetic":false,"types":[]},{"text":"impl Copy for Elf64_Shdr","synthetic":false,"types":[]},{"text":"impl Copy for Elf32_Chdr","synthetic":false,"types":[]},{"text":"impl Copy for Elf64_Chdr","synthetic":false,"types":[]},{"text":"impl Copy for ucred","synthetic":false,"types":[]},{"text":"impl Copy for mntent","synthetic":false,"types":[]},{"text":"impl Copy for posix_spawn_file_actions_t","synthetic":false,"types":[]},{"text":"impl Copy for posix_spawnattr_t","synthetic":false,"types":[]},{"text":"impl Copy for genlmsghdr","synthetic":false,"types":[]},{"text":"impl Copy for in6_pktinfo","synthetic":false,"types":[]},{"text":"impl Copy for arpd_request","synthetic":false,"types":[]},{"text":"impl Copy for inotify_event","synthetic":false,"types":[]},{"text":"impl Copy for fanotify_response","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_vm","synthetic":false,"types":[]},{"text":"impl Copy for regmatch_t","synthetic":false,"types":[]},{"text":"impl Copy for sock_extended_err","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_nl","synthetic":false,"types":[]},{"text":"impl Copy for dirent","synthetic":false,"types":[]},{"text":"impl Copy for dirent64","synthetic":false,"types":[]},{"text":"impl Copy for sockaddr_alg","synthetic":false,"types":[]},{"text":"impl Copy for af_alg_iv","synthetic":false,"types":[]},{"text":"impl Copy for mq_attr","synthetic":false,"types":[]},{"text":"impl Copy for statx","synthetic":false,"types":[]},{"text":"impl Copy for statx_timestamp","synthetic":false,"types":[]},{"text":"impl Copy for aiocb","synthetic":false,"types":[]},{"text":"impl Copy for __exit_status","synthetic":false,"types":[]},{"text":"impl Copy for __timeval","synthetic":false,"types":[]},{"text":"impl Copy for glob64_t","synthetic":false,"types":[]},{"text":"impl Copy for msghdr","synthetic":false,"types":[]},{"text":"impl Copy for cmsghdr","synthetic":false,"types":[]},{"text":"impl Copy for termios","synthetic":false,"types":[]},{"text":"impl Copy for mallinfo","synthetic":false,"types":[]},{"text":"impl Copy for nlmsghdr","synthetic":false,"types":[]},{"text":"impl Copy for nlmsgerr","synthetic":false,"types":[]},{"text":"impl Copy for nl_pktinfo","synthetic":false,"types":[]},{"text":"impl Copy for nl_mmap_req","synthetic":false,"types":[]},{"text":"impl Copy for nl_mmap_hdr","synthetic":false,"types":[]},{"text":"impl Copy for nlattr","synthetic":false,"types":[]},{"text":"impl Copy for rtentry","synthetic":false,"types":[]},{"text":"impl Copy for timex","synthetic":false,"types":[]},{"text":"impl Copy for ntptimeval","synthetic":false,"types":[]},{"text":"impl Copy for regex_t","synthetic":false,"types":[]},{"text":"impl Copy for utmpx","synthetic":false,"types":[]},{"text":"impl Copy for sigset_t","synthetic":false,"types":[]},{"text":"impl Copy for sysinfo","synthetic":false,"types":[]},{"text":"impl Copy for msqid_ds","synthetic":false,"types":[]},{"text":"impl Copy for sigaction","synthetic":false,"types":[]},{"text":"impl Copy for statfs","synthetic":false,"types":[]},{"text":"impl Copy for flock","synthetic":false,"types":[]},{"text":"impl Copy for flock64","synthetic":false,"types":[]},{"text":"impl Copy for siginfo_t","synthetic":false,"types":[]},{"text":"impl Copy for stack_t","synthetic":false,"types":[]},{"text":"impl Copy for stat","synthetic":false,"types":[]},{"text":"impl Copy for stat64","synthetic":false,"types":[]},{"text":"impl Copy for statfs64","synthetic":false,"types":[]},{"text":"impl Copy for statvfs64","synthetic":false,"types":[]},{"text":"impl Copy for pthread_attr_t","synthetic":false,"types":[]},{"text":"impl Copy for _libc_fpxreg","synthetic":false,"types":[]},{"text":"impl Copy for _libc_xmmreg","synthetic":false,"types":[]},{"text":"impl Copy for _libc_fpstate","synthetic":false,"types":[]},{"text":"impl Copy for user_regs_struct","synthetic":false,"types":[]},{"text":"impl Copy for user","synthetic":false,"types":[]},{"text":"impl Copy for mcontext_t","synthetic":false,"types":[]},{"text":"impl Copy for ipc_perm","synthetic":false,"types":[]},{"text":"impl Copy for shmid_ds","synthetic":false,"types":[]},{"text":"impl Copy for termios2","synthetic":false,"types":[]},{"text":"impl Copy for ip_mreqn","synthetic":false,"types":[]},{"text":"impl Copy for user_fpregs_struct","synthetic":false,"types":[]},{"text":"impl Copy for ucontext_t","synthetic":false,"types":[]},{"text":"impl Copy for statvfs","synthetic":false,"types":[]},{"text":"impl Copy for max_align_t","synthetic":false,"types":[]},{"text":"impl Copy for sem_t","synthetic":false,"types":[]},{"text":"impl Copy for pthread_mutexattr_t","synthetic":false,"types":[]},{"text":"impl Copy for pthread_rwlockattr_t","synthetic":false,"types":[]},{"text":"impl Copy for pthread_condattr_t","synthetic":false,"types":[]},{"text":"impl Copy for fanotify_event_metadata","synthetic":false,"types":[]},{"text":"impl Copy for pthread_cond_t","synthetic":false,"types":[]},{"text":"impl Copy for pthread_mutex_t","synthetic":false,"types":[]},{"text":"impl Copy for pthread_rwlock_t","synthetic":false,"types":[]},{"text":"impl Copy for in6_addr","synthetic":false,"types":[]}];
implementors["miniz_oxide"] = [{"text":"impl Copy for CompressionStrategy","synthetic":false,"types":[]},{"text":"impl Copy for TDEFLFlush","synthetic":false,"types":[]},{"text":"impl Copy for TDEFLStatus","synthetic":false,"types":[]},{"text":"impl Copy for CompressionLevel","synthetic":false,"types":[]},{"text":"impl Copy for TINFLStatus","synthetic":false,"types":[]},{"text":"impl Copy for MZFlush","synthetic":false,"types":[]},{"text":"impl Copy for MZStatus","synthetic":false,"types":[]},{"text":"impl Copy for MZError","synthetic":false,"types":[]},{"text":"impl Copy for DataFormat","synthetic":false,"types":[]},{"text":"impl Copy for StreamResult","synthetic":false,"types":[]}];
implementors["num_integer"] = [{"text":"impl&lt;A:&nbsp;Copy&gt; Copy for ExtendedGcd&lt;A&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Copy&gt; Copy for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Copy for ParseRatioError","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl Copy for ColorType","synthetic":false,"types":[]},{"text":"impl Copy for BitDepth","synthetic":false,"types":[]},{"text":"impl Copy for PixelDimensions","synthetic":false,"types":[]},{"text":"impl Copy for Unit","synthetic":false,"types":[]},{"text":"impl Copy for DisposeOp","synthetic":false,"types":[]},{"text":"impl Copy for BlendOp","synthetic":false,"types":[]},{"text":"impl Copy for FrameControl","synthetic":false,"types":[]},{"text":"impl Copy for AnimationControl","synthetic":false,"types":[]},{"text":"impl Copy for Transformations","synthetic":false,"types":[]},{"text":"impl Copy for Limits","synthetic":false,"types":[]},{"text":"impl Copy for FilterType","synthetic":false,"types":[]}];
implementors["regex"] = [{"text":"impl&lt;'t&gt; Copy for Match&lt;'t&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'t&gt; Copy for Match&lt;'t&gt;","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl Copy for Span","synthetic":false,"types":[]},{"text":"impl Copy for Position","synthetic":false,"types":[]},{"text":"impl Copy for ClassSetBinaryOpKind","synthetic":false,"types":[]},{"text":"impl Copy for Flag","synthetic":false,"types":[]},{"text":"impl Copy for ClassUnicodeRange","synthetic":false,"types":[]},{"text":"impl Copy for ClassBytesRange","synthetic":false,"types":[]},{"text":"impl Copy for Utf8Sequence","synthetic":false,"types":[]},{"text":"impl Copy for Utf8Range","synthetic":false,"types":[]}];
implementors["termimage"] = [{"text":"impl Copy for AnsiOutputFormat","synthetic":false,"types":[]}];
implementors["tiff"] = [{"text":"impl Copy for Tag","synthetic":false,"types":[]},{"text":"impl Copy for Type","synthetic":false,"types":[]},{"text":"impl Copy for CompressionMethod","synthetic":false,"types":[]},{"text":"impl Copy for PhotometricInterpretation","synthetic":false,"types":[]},{"text":"impl Copy for PlanarConfiguration","synthetic":false,"types":[]},{"text":"impl Copy for Predictor","synthetic":false,"types":[]},{"text":"impl Copy for ResolutionUnit","synthetic":false,"types":[]},{"text":"impl Copy for SampleFormat","synthetic":false,"types":[]},{"text":"impl Copy for ColorType","synthetic":false,"types":[]}];
implementors["weezl"] = [{"text":"impl Copy for LzwStatus","synthetic":false,"types":[]},{"text":"impl Copy for LzwError","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()