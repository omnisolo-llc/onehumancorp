#[derive(Debug, Clone, PartialEq)]
pub enum PaytmMethod {
    Wallet,
    UPI,
    NetBanking,
}

pub struct PaytmRouter;
impl PaytmRouter {
    pub fn optimize_payment_method(amount_inr: f64) -> PaytmMethod {
        if amount_inr < 1000.0 {
            PaytmMethod::UPI
        } else {
            PaytmMethod::NetBanking
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_paytm_routing_logic_1() {
        let method = PaytmRouter::optimize_payment_method(10.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_2() {
        let method = PaytmRouter::optimize_payment_method(20.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_3() {
        let method = PaytmRouter::optimize_payment_method(30.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_4() {
        let method = PaytmRouter::optimize_payment_method(40.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_5() {
        let method = PaytmRouter::optimize_payment_method(50.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_6() {
        let method = PaytmRouter::optimize_payment_method(60.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_7() {
        let method = PaytmRouter::optimize_payment_method(70.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_8() {
        let method = PaytmRouter::optimize_payment_method(80.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_9() {
        let method = PaytmRouter::optimize_payment_method(90.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_10() {
        let method = PaytmRouter::optimize_payment_method(100.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_11() {
        let method = PaytmRouter::optimize_payment_method(110.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_12() {
        let method = PaytmRouter::optimize_payment_method(120.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_13() {
        let method = PaytmRouter::optimize_payment_method(130.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_14() {
        let method = PaytmRouter::optimize_payment_method(140.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_15() {
        let method = PaytmRouter::optimize_payment_method(150.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_16() {
        let method = PaytmRouter::optimize_payment_method(160.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_17() {
        let method = PaytmRouter::optimize_payment_method(170.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_18() {
        let method = PaytmRouter::optimize_payment_method(180.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_19() {
        let method = PaytmRouter::optimize_payment_method(190.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_20() {
        let method = PaytmRouter::optimize_payment_method(200.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_21() {
        let method = PaytmRouter::optimize_payment_method(210.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_22() {
        let method = PaytmRouter::optimize_payment_method(220.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_23() {
        let method = PaytmRouter::optimize_payment_method(230.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_24() {
        let method = PaytmRouter::optimize_payment_method(240.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_25() {
        let method = PaytmRouter::optimize_payment_method(250.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_26() {
        let method = PaytmRouter::optimize_payment_method(260.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_27() {
        let method = PaytmRouter::optimize_payment_method(270.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_28() {
        let method = PaytmRouter::optimize_payment_method(280.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_29() {
        let method = PaytmRouter::optimize_payment_method(290.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_30() {
        let method = PaytmRouter::optimize_payment_method(300.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_31() {
        let method = PaytmRouter::optimize_payment_method(310.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_32() {
        let method = PaytmRouter::optimize_payment_method(320.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_33() {
        let method = PaytmRouter::optimize_payment_method(330.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_34() {
        let method = PaytmRouter::optimize_payment_method(340.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_35() {
        let method = PaytmRouter::optimize_payment_method(350.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_36() {
        let method = PaytmRouter::optimize_payment_method(360.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_37() {
        let method = PaytmRouter::optimize_payment_method(370.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_38() {
        let method = PaytmRouter::optimize_payment_method(380.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_39() {
        let method = PaytmRouter::optimize_payment_method(390.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_40() {
        let method = PaytmRouter::optimize_payment_method(400.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_41() {
        let method = PaytmRouter::optimize_payment_method(410.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_42() {
        let method = PaytmRouter::optimize_payment_method(420.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_43() {
        let method = PaytmRouter::optimize_payment_method(430.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_44() {
        let method = PaytmRouter::optimize_payment_method(440.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_45() {
        let method = PaytmRouter::optimize_payment_method(450.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_46() {
        let method = PaytmRouter::optimize_payment_method(460.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_47() {
        let method = PaytmRouter::optimize_payment_method(470.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_48() {
        let method = PaytmRouter::optimize_payment_method(480.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_49() {
        let method = PaytmRouter::optimize_payment_method(490.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_50() {
        let method = PaytmRouter::optimize_payment_method(500.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_51() {
        let method = PaytmRouter::optimize_payment_method(510.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_52() {
        let method = PaytmRouter::optimize_payment_method(520.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_53() {
        let method = PaytmRouter::optimize_payment_method(530.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_54() {
        let method = PaytmRouter::optimize_payment_method(540.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_55() {
        let method = PaytmRouter::optimize_payment_method(550.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_56() {
        let method = PaytmRouter::optimize_payment_method(560.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_57() {
        let method = PaytmRouter::optimize_payment_method(570.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_58() {
        let method = PaytmRouter::optimize_payment_method(580.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_59() {
        let method = PaytmRouter::optimize_payment_method(590.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_60() {
        let method = PaytmRouter::optimize_payment_method(600.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_61() {
        let method = PaytmRouter::optimize_payment_method(610.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_62() {
        let method = PaytmRouter::optimize_payment_method(620.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_63() {
        let method = PaytmRouter::optimize_payment_method(630.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_64() {
        let method = PaytmRouter::optimize_payment_method(640.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_65() {
        let method = PaytmRouter::optimize_payment_method(650.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_66() {
        let method = PaytmRouter::optimize_payment_method(660.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_67() {
        let method = PaytmRouter::optimize_payment_method(670.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_68() {
        let method = PaytmRouter::optimize_payment_method(680.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_69() {
        let method = PaytmRouter::optimize_payment_method(690.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_70() {
        let method = PaytmRouter::optimize_payment_method(700.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_71() {
        let method = PaytmRouter::optimize_payment_method(710.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_72() {
        let method = PaytmRouter::optimize_payment_method(720.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_73() {
        let method = PaytmRouter::optimize_payment_method(730.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_74() {
        let method = PaytmRouter::optimize_payment_method(740.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_75() {
        let method = PaytmRouter::optimize_payment_method(750.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_76() {
        let method = PaytmRouter::optimize_payment_method(760.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_77() {
        let method = PaytmRouter::optimize_payment_method(770.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_78() {
        let method = PaytmRouter::optimize_payment_method(780.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_79() {
        let method = PaytmRouter::optimize_payment_method(790.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_80() {
        let method = PaytmRouter::optimize_payment_method(800.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_81() {
        let method = PaytmRouter::optimize_payment_method(810.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_82() {
        let method = PaytmRouter::optimize_payment_method(820.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_83() {
        let method = PaytmRouter::optimize_payment_method(830.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_84() {
        let method = PaytmRouter::optimize_payment_method(840.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_85() {
        let method = PaytmRouter::optimize_payment_method(850.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_86() {
        let method = PaytmRouter::optimize_payment_method(860.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_87() {
        let method = PaytmRouter::optimize_payment_method(870.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_88() {
        let method = PaytmRouter::optimize_payment_method(880.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_89() {
        let method = PaytmRouter::optimize_payment_method(890.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_90() {
        let method = PaytmRouter::optimize_payment_method(900.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_91() {
        let method = PaytmRouter::optimize_payment_method(910.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_92() {
        let method = PaytmRouter::optimize_payment_method(920.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_93() {
        let method = PaytmRouter::optimize_payment_method(930.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_94() {
        let method = PaytmRouter::optimize_payment_method(940.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_95() {
        let method = PaytmRouter::optimize_payment_method(950.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_96() {
        let method = PaytmRouter::optimize_payment_method(960.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_97() {
        let method = PaytmRouter::optimize_payment_method(970.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_98() {
        let method = PaytmRouter::optimize_payment_method(980.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_99() {
        let method = PaytmRouter::optimize_payment_method(990.0);
        assert_eq!(method, PaytmMethod::UPI);
    }

    #[test]
    fn test_paytm_routing_logic_100() {
        let method = PaytmRouter::optimize_payment_method(1000.0);
        assert_eq!(method, PaytmMethod::NetBanking);
    }

}
