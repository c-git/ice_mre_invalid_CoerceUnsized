# ice_mre_invalid_CoerceUnsized

Created from https://github.com/skifli/bruty at https://github.com/skifli/bruty/commit/973a536b7dd3f981f2dc6b4a07fb76a7a1040975
Stripped down within time available

Running `cargo check` succeeds but `cargo build` gives the following internal compiler error (ICE):

```sh
   Compiling ice_mre_invalid_coerce_unsized v0.1.0 (/home/one/ice_mre_invalid_CoerceUnsized)
error: internal compiler error: compiler/rustc_monomorphize/src/lib.rs:46:13: invalid `CoerceUnsized` impl_source: Err(FulfillmentError)

thread 'rustc' panicked at compiler/rustc_monomorphize/src/lib.rs:46:13:
Box<dyn Any>
stack backtrace:
   0:     0x76711618682a - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::ha4a311b32f6b4ad8
   1:     0x7671168277e2 - core::fmt::write::h1866771663f62b81
   2:     0x767117681d51 - std::io::Write::write_fmt::hb549e7444823135e
   3:     0x767116186682 - std::sys::backtrace::BacktraceLock::print::hddd3a9918ce29aa7
   4:     0x767116188b5a - std::panicking::default_hook::{{closure}}::h791f75256b902d7d
   5:     0x7671161889c0 - std::panicking::default_hook::h82cc572fcb0d8cd7
   6:     0x767115214f55 - std[1b49f43dde054edc]::panicking::update_hook::<alloc[f0e0d4128a1437e6]::boxed::Box<rustc_driver_impl[c421ed190efad9be]::install_ice_hook::{closure#0}>>::{closure#0}
   7:     0x767116189238 - std::panicking::rust_panic_with_hook::he21644cc2707f2c4
   8:     0x76711524fa01 - std[1b49f43dde054edc]::panicking::begin_panic::<rustc_errors[fd0d1ab268a7514d]::ExplicitBug>::{closure#0}
   9:     0x7671152429c6 - std[1b49f43dde054edc]::sys::backtrace::__rust_end_short_backtrace::<std[1b49f43dde054edc]::panicking::begin_panic<rustc_errors[fd0d1ab268a7514d]::ExplicitBug>::{closure#0}, !>
  10:     0x76711523df99 - std[1b49f43dde054edc]::panicking::begin_panic::<rustc_errors[fd0d1ab268a7514d]::ExplicitBug>
  11:     0x767115259971 - <rustc_errors[fd0d1ab268a7514d]::diagnostic::BugAbort as rustc_errors[fd0d1ab268a7514d]::diagnostic::EmissionGuarantee>::emit_producing_guarantee
  12:     0x7671158c8913 - rustc_middle[60437f3b60b3af56]::util::bug::opt_span_bug_fmt::<rustc_span[200b27ea0e9a3b9b]::span_encoding::Span>::{closure#0}
  13:     0x7671158afe8a - rustc_middle[60437f3b60b3af56]::ty::context::tls::with_opt::<rustc_middle[60437f3b60b3af56]::util::bug::opt_span_bug_fmt<rustc_span[200b27ea0e9a3b9b]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  14:     0x7671158afd1b - rustc_middle[60437f3b60b3af56]::ty::context::tls::with_context_opt::<rustc_middle[60437f3b60b3af56]::ty::context::tls::with_opt<rustc_middle[60437f3b60b3af56]::util::bug::opt_span_bug_fmt<rustc_span[200b27ea0e9a3b9b]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  15:     0x767113de4110 - rustc_middle[60437f3b60b3af56]::util::bug::bug_fmt
  16:     0x767118255c2d - rustc_monomorphize[64293748b2428815]::collector::find_vtable_types_for_unsizing.cold
  17:     0x767114066669 - rustc_monomorphize[64293748b2428815]::collector::items_of_instance
  18:     0x767116ed92b2 - rustc_query_impl[d10191050d412fc]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[d10191050d412fc]::query_impl::items_of_instance::dynamic_query::{closure#2}::{closure#0}, rustc_middle[60437f3b60b3af56]::query::erase::Erased<[u8; 32usize]>>
  19:     0x767116ed73b4 - rustc_query_system[c1574a252f7419c]::query::plumbing::try_execute_query::<rustc_query_impl[d10191050d412fc]::DynamicConfig<rustc_query_system[c1574a252f7419c]::query::caches::DefaultCache<(rustc_middle[60437f3b60b3af56]::ty::instance::Instance, rustc_middle[60437f3b60b3af56]::mir::mono::CollectionMode), rustc_middle[60437f3b60b3af56]::query::erase::Erased<[u8; 32usize]>>, false, false, false>, rustc_query_impl[d10191050d412fc]::plumbing::QueryCtxt, true>
  20:     0x767116ed5d18 - rustc_query_impl[d10191050d412fc]::query_impl::items_of_instance::get_query_incr::__rust_end_short_backtrace
  21:     0x767116ed3572 - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec::{closure#0}
  22:     0x767117a33048 - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  23:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  24:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  25:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  26:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  27:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  28:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  29:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  30:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  31:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  32:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  33:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  34:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  35:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  36:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  37:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  38:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  39:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  40:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  41:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  42:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  43:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  44:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  45:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  46:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  47:     0x767117a3399e - rustc_monomorphize[64293748b2428815]::collector::collect_items_rec
  48:     0x767117248de9 - rustc_monomorphize[64293748b2428815]::partitioning::collect_and_partition_mono_items
  49:     0x767117248416 - rustc_query_impl[d10191050d412fc]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[d10191050d412fc]::query_impl::collect_and_partition_mono_items::dynamic_query::{closure#2}::{closure#0}, rustc_middle[60437f3b60b3af56]::query::erase::Erased<[u8; 24usize]>>
  50:     0x7671172483e3 - <rustc_query_impl[d10191050d412fc]::query_impl::collect_and_partition_mono_items::dynamic_query::{closure#2} as core[9e3ec3a99e20741e]::ops::function::FnOnce<(rustc_middle[60437f3b60b3af56]::ty::context::TyCtxt, ())>>::call_once
  51:     0x7671177255c0 - rustc_query_system[c1574a252f7419c]::query::plumbing::try_execute_query::<rustc_query_impl[d10191050d412fc]::DynamicConfig<rustc_query_system[c1574a252f7419c]::query::caches::SingleCache<rustc_middle[60437f3b60b3af56]::query::erase::Erased<[u8; 24usize]>>, false, false, false>, rustc_query_impl[d10191050d412fc]::plumbing::QueryCtxt, true>
  52:     0x767117724f09 - rustc_query_impl[d10191050d412fc]::query_impl::collect_and_partition_mono_items::get_query_incr::__rust_end_short_backtrace
  53:     0x76711777c547 - <rustc_codegen_llvm[87a67cd1a6f247bf]::LlvmCodegenBackend as rustc_codegen_ssa[47ed54211a626f01]::traits::backend::CodegenBackend>::codegen_crate
  54:     0x76711778e327 - <rustc_interface[aa3cb6198a62650b]::queries::Linker>::codegen_and_build_linker
  55:     0x7671176652c8 - rustc_interface[aa3cb6198a62650b]::interface::run_compiler::<core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>, rustc_driver_impl[c421ed190efad9be]::run_compiler::{closure#0}>::{closure#1}
  56:     0x7671176a01e0 - std[1b49f43dde054edc]::sys::backtrace::__rust_begin_short_backtrace::<rustc_interface[aa3cb6198a62650b]::util::run_in_thread_with_globals<rustc_interface[aa3cb6198a62650b]::util::run_in_thread_pool_with_globals<rustc_interface[aa3cb6198a62650b]::interface::run_compiler<core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>, rustc_driver_impl[c421ed190efad9be]::run_compiler::{closure#0}>::{closure#1}, core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>>::{closure#0}, core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>>
  57:     0x76711769fefd - <<std[1b49f43dde054edc]::thread::Builder>::spawn_unchecked_<rustc_interface[aa3cb6198a62650b]::util::run_in_thread_with_globals<rustc_interface[aa3cb6198a62650b]::util::run_in_thread_pool_with_globals<rustc_interface[aa3cb6198a62650b]::interface::run_compiler<core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>, rustc_driver_impl[c421ed190efad9be]::run_compiler::{closure#0}>::{closure#1}, core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>>::{closure#0}, core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e3ec3a99e20741e]::result::Result<(), rustc_span[200b27ea0e9a3b9b]::ErrorGuaranteed>>::{closure#1} as core[9e3ec3a99e20741e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x76711769f6b9 - std::sys::pal::unix::thread::Thread::new::thread_start::h14f1eb868ff90fc9
  59:     0x767111894ac3 - start_thread
                               at ./nptl/pthread_create.c:442:8
  60:     0x767111926850 - __GI___clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  61:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.84.0 (9fc6b4312 2025-01-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [items_of_instance] collecting items used by `<shuttle_proto::generated::runtime::runtime_server::RuntimeServer<shuttle_runtime::alpha::Alpha<__loader, __runner>> as tower_service::Service<http::request::Request<hyper::body::body::Body>>>::call`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `ice_mre_invalid_coerce_unsized` (bin "ice_mre_invalid_coerce_unsized")

Caused by:
  process didn't exit successfully: `/home/one/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name ice_mre_invalid_coerce_unsized --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs)' --check-cfg 'cfg(feature, values())' -C metadata=b41eeaf4ed19f76e -C extra-filename=-b41eeaf4ed19f76e --out-dir /home/one/ice_mre_invalid_CoerceUnsized/target/debug/deps -C incremental=/home/one/ice_mre_invalid_CoerceUnsized/target/debug/incremental -L dependency=/home/one/ice_mre_invalid_CoerceUnsized/target/debug/deps --extern shuttle_runtime=/home/one/ice_mre_invalid_CoerceUnsized/target/debug/deps/libshuttle_runtime-97b95e87d497367a.rlib --extern shuttle_shared_db=/home/one/ice_mre_invalid_CoerceUnsized/target/debug/deps/libshuttle_shared_db-a978a2d27ac6357d.rlib -L native=/home/one/ice_mre_invalid_CoerceUnsized/target/debug/build/ring-c42c0ae835c8384f/out` (exit status: 101)
```