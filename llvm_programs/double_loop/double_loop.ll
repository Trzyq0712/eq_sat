; ModuleID = 'double_loop.c'
source_filename = "double_loop.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i64 @double_loop(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  store i64 0, ptr %3, align 8
  store i64 0, ptr %4, align 8
  br label %6

6:                                                ; preds = %25, %1
  %7 = load i64, ptr %4, align 8
  %8 = load i64, ptr %2, align 8
  %9 = icmp slt i64 %7, %8
  br i1 %9, label %10, label %28

10:                                               ; preds = %6
  store i64 0, ptr %5, align 8
  br label %11

11:                                               ; preds = %21, %10
  %12 = load i64, ptr %5, align 8
  %13 = load i64, ptr %2, align 8
  %14 = icmp slt i64 %12, %13
  br i1 %14, label %15, label %24

15:                                               ; preds = %11
  %16 = load i64, ptr %4, align 8
  %17 = load i64, ptr %5, align 8
  %18 = mul nsw i64 %16, %17
  %19 = load i64, ptr %3, align 8
  %20 = add nsw i64 %19, %18
  store i64 %20, ptr %3, align 8
  br label %21

21:                                               ; preds = %15
  %22 = load i64, ptr %5, align 8
  %23 = add nsw i64 %22, 1
  store i64 %23, ptr %5, align 8
  br label %11, !llvm.loop !6

24:                                               ; preds = %11
  br label %25

25:                                               ; preds = %24
  %26 = load i64, ptr %4, align 8
  %27 = add nsw i64 %26, 1
  store i64 %27, ptr %4, align 8
  br label %6, !llvm.loop !8

28:                                               ; preds = %6
  %29 = load i64, ptr %3, align 8
  ret i64 %29
}

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"clang version 15.0.7"}
!6 = distinct !{!6, !7}
!7 = !{!"llvm.loop.mustprogress"}
!8 = distinct !{!8, !7}
