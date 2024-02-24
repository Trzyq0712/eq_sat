; ModuleID = 'nested_loop.c'
source_filename = "nested_loop.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i64 @gauss_sum(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  store i64 0, ptr %3, align 8
  store i64 1, ptr %4, align 8
  br label %6

6:                                                ; preds = %22, %1
  %7 = load i64, ptr %4, align 8
  %8 = load i64, ptr %2, align 8
  %9 = icmp sle i64 %7, %8
  br i1 %9, label %10, label %25

10:                                               ; preds = %6
  store i64 1, ptr %5, align 8
  br label %11

11:                                               ; preds = %18, %10
  %12 = load i64, ptr %5, align 8
  %13 = load i64, ptr %4, align 8
  %14 = icmp sle i64 %12, %13
  br i1 %14, label %15, label %21

15:                                               ; preds = %11
  %16 = load i64, ptr %3, align 8
  %17 = add nsw i64 %16, 1
  store i64 %17, ptr %3, align 8
  br label %18

18:                                               ; preds = %15
  %19 = load i64, ptr %5, align 8
  %20 = add nsw i64 %19, 1
  store i64 %20, ptr %5, align 8
  br label %11, !llvm.loop !6

21:                                               ; preds = %11
  br label %22

22:                                               ; preds = %21
  %23 = load i64, ptr %4, align 8
  %24 = add nsw i64 %23, 1
  store i64 %24, ptr %4, align 8
  br label %6, !llvm.loop !8

25:                                               ; preds = %6
  %26 = load i64, ptr %3, align 8
  ret i64 %26
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
