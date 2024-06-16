#[test]
fn feature() {
    let bets = [10000_f32, 100.0, 100.0, 1000.0, 100.0, 100.0, 100.0, 1000.0];
    let pays  = [0.0, 200.0, 200.0, 200.0, 200.0, 200.0, 200.0, 200.0, 200.0, 0.0];
    let mut total_bet = 0_f32;
    let mut total_pay = 0_f32;
    
    bets.iter().zip(pays.iter()).for_each(|(bet, pay)| {
        total_bet += *bet;
        total_pay += *pay;
        let diff = total_bet - total_pay;
        let diff_percent = total_bet / total_pay * 100.0;
        eprintln!("bet: {:5}, pay: {:5}, 库存: {:6}, RTP: {:.2}%", bet, pay, diff, diff_percent);
        
    });


}