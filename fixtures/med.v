module a_not_b(input i_A, input i_B, output o_led);
  /* reg state; */

  /* always @(posedge i_clk) */
  /*   state <= !state; */

  assign o_led = (i_A == 1'b1) && (i_B == 1'b0);

endmodule

module top(input A, input B);
  a_not_b m1 (A, B, m2_A);
  a_not_b m2 (m2_A, B, B);
endmodule
