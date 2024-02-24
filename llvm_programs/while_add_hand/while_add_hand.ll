define i64 @while_add_hand(i64 %a) {
  %i_p = alloca i64
  store i64 0, ptr %i_p
  br label %loop.head 

loop.head:
  %i = load i64, ptr %i_p
  %cond = icmp slt i64 %i, %a
  br i1 %cond, label %loop.body, label %loop.end

loop.body:
  %i1 = load i64, ptr %i_p
  %i2 = add i64 %i1, 1
  store i64 %i2, ptr %i_p
  br label %loop.head

loop.end:
  %res = load i64, ptr %i_p
  ret i64 %res
}
