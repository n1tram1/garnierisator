module a_not_b(input i_A, input i_B, output o_led);
  assign o_led = (i_A == 1'b1) && (i_B == 1'b0);
endmodule

module top(A, B, o_m1, o_m2);
  input A;
  input B;
  output o_m1;
  output o_m2;
  wire m2_A;

  a_not_b m1 (
    A,
    B,
    m2_A
  );

  a_not_b m2 (
    m2_A,
    B,
    o_m2,
  );

  assign o_m1 = m2_A;
endmodule
