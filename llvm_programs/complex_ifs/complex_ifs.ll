; ModuleID = 'complex_ifs.c'
source_filename = "complex_ifs.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i64 @complex_ifs(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  store i64 0, ptr %7, align 8
  %8 = load i64, ptr %7, align 8
  %9 = add nsw i64 %8, 2
  store i64 %9, ptr %7, align 8
  %10 = load i64, ptr %4, align 8
  %11 = load i64, ptr %5, align 8
  %12 = icmp sgt i64 %10, %11
  br i1 %12, label %13, label %16

13:                                               ; preds = %3
  %14 = load i64, ptr %7, align 8
  %15 = sub nsw i64 %14, 3
  store i64 %15, ptr %7, align 8
  br label %26

16:                                               ; preds = %3
  %17 = load i64, ptr %7, align 8
  %18 = add nsw i64 %17, 5
  store i64 %18, ptr %7, align 8
  %19 = load i64, ptr %7, align 8
  %20 = load i64, ptr %6, align 8
  %21 = icmp sgt i64 %19, %20
  br i1 %21, label %22, label %25

22:                                               ; preds = %16
  %23 = load i64, ptr %7, align 8
  %24 = add nsw i64 %23, 7
  store i64 %24, ptr %7, align 8
  br label %25

25:                                               ; preds = %22, %16
  br label %26

26:                                               ; preds = %25, %13
  %27 = load i64, ptr %7, align 8
  %28 = mul nsw i64 %27, 10
  store i64 %28, ptr %7, align 8
  %29 = load i64, ptr %7, align 8
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
