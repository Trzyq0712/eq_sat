; ModuleID = 'double_loop.c'
source_filename = "double_loop.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i64 @double_loop(i64 noundef %0) #0 !dbg !10 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  call void @llvm.dbg.declare(metadata ptr %2, metadata !19, metadata !DIExpression()), !dbg !20
  call void @llvm.dbg.declare(metadata ptr %3, metadata !21, metadata !DIExpression()), !dbg !22
  store i64 0, ptr %3, align 8, !dbg !22
  call void @llvm.dbg.declare(metadata ptr %4, metadata !23, metadata !DIExpression()), !dbg !25
  store i64 0, ptr %4, align 8, !dbg !25
  br label %6, !dbg !26

6:                                                ; preds = %25, %1
  %7 = load i64, ptr %4, align 8, !dbg !27
  %8 = load i64, ptr %2, align 8, !dbg !29
  %9 = icmp slt i64 %7, %8, !dbg !30
  br i1 %9, label %10, label %28, !dbg !31

10:                                               ; preds = %6
  call void @llvm.dbg.declare(metadata ptr %5, metadata !32, metadata !DIExpression()), !dbg !35
  store i64 0, ptr %5, align 8, !dbg !35
  br label %11, !dbg !36

11:                                               ; preds = %21, %10
  %12 = load i64, ptr %5, align 8, !dbg !37
  %13 = load i64, ptr %2, align 8, !dbg !39
  %14 = icmp slt i64 %12, %13, !dbg !40
  br i1 %14, label %15, label %24, !dbg !41

15:                                               ; preds = %11
  %16 = load i64, ptr %4, align 8, !dbg !42
  %17 = load i64, ptr %5, align 8, !dbg !44
  %18 = mul nsw i64 %16, %17, !dbg !45
  %19 = load i64, ptr %3, align 8, !dbg !46
  %20 = add nsw i64 %19, %18, !dbg !46
  store i64 %20, ptr %3, align 8, !dbg !46
  br label %21, !dbg !47

21:                                               ; preds = %15
  %22 = load i64, ptr %5, align 8, !dbg !48
  %23 = add nsw i64 %22, 1, !dbg !48
  store i64 %23, ptr %5, align 8, !dbg !48
  br label %11, !dbg !49, !llvm.loop !50

24:                                               ; preds = %11
  br label %25, !dbg !53

25:                                               ; preds = %24
  %26 = load i64, ptr %4, align 8, !dbg !54
  %27 = add nsw i64 %26, 1, !dbg !54
  store i64 %27, ptr %4, align 8, !dbg !54
  br label %6, !dbg !55, !llvm.loop !56

28:                                               ; preds = %6
  %29 = load i64, ptr %3, align 8, !dbg !58
  ret i64 %29, !dbg !59
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6, !7, !8}
!llvm.ident = !{!9}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "clang version 15.0.7", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "double_loop.c", directory: "/home/trzyq/Documents/tudelft/honours/eq_satur/eq_sat/llvm_programs/double_loop", checksumkind: CSK_MD5, checksum: "3a0412bf812446875ed34ec96fb3d09f")
!2 = !{i32 7, !"Dwarf Version", i32 5}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{i32 1, !"wchar_size", i32 4}
!5 = !{i32 7, !"PIC Level", i32 2}
!6 = !{i32 7, !"PIE Level", i32 2}
!7 = !{i32 7, !"uwtable", i32 2}
!8 = !{i32 7, !"frame-pointer", i32 2}
!9 = !{!"clang version 15.0.7"}
!10 = distinct !DISubprogram(name: "double_loop", scope: !1, file: !1, line: 3, type: !11, scopeLine: 3, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !18)
!11 = !DISubroutineType(types: !12)
!12 = !{!13, !13}
!13 = !DIDerivedType(tag: DW_TAG_typedef, name: "int64_t", file: !14, line: 27, baseType: !15)
!14 = !DIFile(filename: "/usr/include/bits/stdint-intn.h", directory: "", checksumkind: CSK_MD5, checksum: "649b383a60bfa3eb90e85840b2b0be20")
!15 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int64_t", file: !16, line: 44, baseType: !17)
!16 = !DIFile(filename: "/usr/include/bits/types.h", directory: "", checksumkind: CSK_MD5, checksum: "e1865d9fe29fe1b5ced550b7ba458f9e")
!17 = !DIBasicType(name: "long", size: 64, encoding: DW_ATE_signed)
!18 = !{}
!19 = !DILocalVariable(name: "n", arg: 1, scope: !10, file: !1, line: 3, type: !13)
!20 = !DILocation(line: 3, column: 29, scope: !10)
!21 = !DILocalVariable(name: "sum", scope: !10, file: !1, line: 4, type: !13)
!22 = !DILocation(line: 4, column: 11, scope: !10)
!23 = !DILocalVariable(name: "i", scope: !24, file: !1, line: 5, type: !13)
!24 = distinct !DILexicalBlock(scope: !10, file: !1, line: 5, column: 3)
!25 = !DILocation(line: 5, column: 16, scope: !24)
!26 = !DILocation(line: 5, column: 8, scope: !24)
!27 = !DILocation(line: 5, column: 23, scope: !28)
!28 = distinct !DILexicalBlock(scope: !24, file: !1, line: 5, column: 3)
!29 = !DILocation(line: 5, column: 27, scope: !28)
!30 = !DILocation(line: 5, column: 25, scope: !28)
!31 = !DILocation(line: 5, column: 3, scope: !24)
!32 = !DILocalVariable(name: "j", scope: !33, file: !1, line: 6, type: !13)
!33 = distinct !DILexicalBlock(scope: !34, file: !1, line: 6, column: 5)
!34 = distinct !DILexicalBlock(scope: !28, file: !1, line: 5, column: 35)
!35 = !DILocation(line: 6, column: 18, scope: !33)
!36 = !DILocation(line: 6, column: 10, scope: !33)
!37 = !DILocation(line: 6, column: 25, scope: !38)
!38 = distinct !DILexicalBlock(scope: !33, file: !1, line: 6, column: 5)
!39 = !DILocation(line: 6, column: 29, scope: !38)
!40 = !DILocation(line: 6, column: 27, scope: !38)
!41 = !DILocation(line: 6, column: 5, scope: !33)
!42 = !DILocation(line: 7, column: 14, scope: !43)
!43 = distinct !DILexicalBlock(scope: !38, file: !1, line: 6, column: 37)
!44 = !DILocation(line: 7, column: 18, scope: !43)
!45 = !DILocation(line: 7, column: 16, scope: !43)
!46 = !DILocation(line: 7, column: 11, scope: !43)
!47 = !DILocation(line: 8, column: 5, scope: !43)
!48 = !DILocation(line: 6, column: 33, scope: !38)
!49 = !DILocation(line: 6, column: 5, scope: !38)
!50 = distinct !{!50, !41, !51, !52}
!51 = !DILocation(line: 8, column: 5, scope: !33)
!52 = !{!"llvm.loop.mustprogress"}
!53 = !DILocation(line: 9, column: 3, scope: !34)
!54 = !DILocation(line: 5, column: 31, scope: !28)
!55 = !DILocation(line: 5, column: 3, scope: !28)
!56 = distinct !{!56, !31, !57, !52}
!57 = !DILocation(line: 9, column: 3, scope: !24)
!58 = !DILocation(line: 10, column: 10, scope: !10)
!59 = !DILocation(line: 10, column: 3, scope: !10)
