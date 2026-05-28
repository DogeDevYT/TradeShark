module D2_circuit (
   input  logic [63:0] d1_in,
   input  logic [63:0] time_expr,
   input  logic [63:0] volatility,
   input  logic        clk,
   input  logic        rst,
   output logic [63:0] d2_out
);

   // σ · √t
   logic [63:0] sqrt_t;
   fp_sqrt sqrtTime (
      .clk   (clk),
      .areset(rst),
      .a     (time_expr),
      .q     (sqrt_t)
   );

   logic [63:0] sigma_sqrt_t;
   fp_multi sigmaSqrtT (
      .clk   (clk),
      .areset(rst),
      .a     (volatility),
      .b     (sqrt_t),
      .q     (sigma_sqrt_t)
   );

   // d2 = d1 - σ·√t
   fp_sub d2Sub (
      .clk   (clk),
      .areset(rst),
      .a     (d1_in),
      .b     (sigma_sqrt_t),
      .q     (d2_out)
   );

endmodule