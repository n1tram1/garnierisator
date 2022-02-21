module blinky(input i_A, input i_B, output o_led);
  assign o_led = (i_A == 1'b1) && (i_B == 1'b0);
endmodule
