module D1_circuit (
   input  logic [63:0] stock_price,   // S
   input  logic [63:0] strike_price,  // K
   input  logic [63:0] time_expr,     // t
   input  logic [63:0] rf_invest,     // r
   input  logic [63:0] volatility,    // sigma
   input  logic        clk,
   input  logic        rst,
   output logic [63:0] d1_out
);

   // Step 1: S / K
   logic [63:0] stck_strike;
   fp_divide stockOverStrike (
      .clk   (clk),
      .areset(rst),
      .a     (stock_price),
      .b     (strike_price),
      .q     (stck_strike)
   );

   // Step 2: ln(S/K)
   logic [63:0] log_stck_strike;
   fp_log logStockOverStrike (
      .clk   (clk),
      .areset(rst),
      .a     (stck_strike),
      .q     (log_stck_strike)
   );

   // Step 3: σ²  (σ * σ)
   logic [63:0] sigma_sqr;
   fp_multi sigmaSqr (
      .clk   (clk),
      .areset(rst),
      .a     (volatility),
      .b     (volatility),
      .q     (sigma_sqr)
   );

   // Step 4: σ² / 2
   // 2.0 in IEEE-754 double = 64'h4000000000000000
   logic [63:0] sigma_sqr_div_2;
   fp_divide sigmaSqrDiv2 (
      .clk   (clk),
      .areset(rst),
      .a     (sigma_sqr),
      .b     (64'h4000000000000000),
      .q     (sigma_sqr_div_2)
   );

   // Step 5: r + σ²/2
   logic [63:0] r_plus_half_sigma_sqr;
   fp_add rPlusHalfSigmaSqr (
      .clk   (clk),
      .areset(rst),
      .a     (rf_invest),
      .b     (sigma_sqr_div_2),
      .q     (r_plus_half_sigma_sqr)
   );

   // Step 6: (r + σ²/2) · t
   logic [63:0] numerator_rhs;
   fp_multi rhs_times_t (
      .clk   (clk),
      .areset(rst),
      .a     (r_plus_half_sigma_sqr),
      .b     (time_expr),
      .q     (numerator_rhs)
   );

   // Step 7: ln(S/K) + (r + σ²/2)·t  → full numerator
   logic [63:0] numerator;
   fp_add numeratorAdd (
      .clk   (clk),
      .areset(rst),
      .a     (log_stck_strike),
      .b     (numerator_rhs),
      .q     (numerator)
   );

   // ── Denominator: σ · √t ─────────────────────────────────────────────

   // Step 8: √t
   logic [63:0] sqrt_t;
   fp_sqrt sqrtTime (
      .clk   (clk),
      .areset(rst),
      .a     (time_expr),
      .q     (sqrt_t)
   );

   // Step 9: σ · √t
   logic [63:0] denominator;
   fp_multi sigmaSqrtT (
      .clk   (clk),
      .areset(rst),
      .a     (volatility),
      .b     (sqrt_t),
      .q     (denominator)
   );

   // ── Final: numerator / denominator ──────────────────────────────────

   fp_divide d1Divide (
      .clk   (clk),
      .areset(rst),
      .a     (numerator),
      .b     (denominator),
      .q     (d1_out)
   );

endmodule