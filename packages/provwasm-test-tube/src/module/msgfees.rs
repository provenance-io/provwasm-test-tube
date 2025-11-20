use provwasm_std::types::provenance::msgfees::v1::{
    MsgAddMsgFeeProposalRequest, MsgAddMsgFeeProposalResponse, MsgAssessCustomMsgFeeRequest,
    MsgAssessCustomMsgFeeResponse, MsgRemoveMsgFeeProposalRequest, MsgRemoveMsgFeeProposalResponse,
    MsgUpdateConversionFeeDenomProposalRequest, MsgUpdateConversionFeeDenomProposalResponse,
    MsgUpdateMsgFeeProposalRequest, MsgUpdateMsgFeeProposalResponse,
    MsgUpdateNhashPerUsdMilProposalRequest, MsgUpdateNhashPerUsdMilProposalResponse,
};
use test_tube_prov::{fn_execute, Module, Runner};

pub struct MsgFees<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for MsgFees<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> MsgFees<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub assess_custom_msg_fee: MsgAssessCustomMsgFeeRequest["/provenance.msgfees.v1.MsgAssessCustomMsgFeeRequest"] => MsgAssessCustomMsgFeeResponse
    }

    fn_execute! {
        pub add_msg_fee_proposal: MsgAddMsgFeeProposalRequest["/provenance.msgfees.v1.MsgAddMsgFeeProposalRequest"] => MsgAddMsgFeeProposalResponse
    }

    fn_execute! {
        pub update_msg_fee_proposal: MsgUpdateMsgFeeProposalRequest["/provenance.msgfees.v1.MsgUpdateMsgFeeProposalRequest"] => MsgUpdateMsgFeeProposalResponse
    }

    fn_execute! {
        pub remove_msg_fee_proposal: MsgRemoveMsgFeeProposalRequest["/provenance.msgfees.v1.MsgRemoveMsgFeeProposalRequest"] => MsgRemoveMsgFeeProposalResponse
    }

    fn_execute! {
        pub update_nhash_per_usd_mil_proposal: MsgUpdateNhashPerUsdMilProposalRequest["/provenance.msgfees.v1.MsgUpdateNhashPerUsdMilProposalRequest"] => MsgUpdateNhashPerUsdMilProposalResponse
    }

    fn_execute! {
        pub update_conversion_fee_denom_proposal: MsgUpdateConversionFeeDenomProposalRequest["/provenance.msgfees.v1.MsgUpdateConversionFeeDenomProposalRequest"] => MsgUpdateConversionFeeDenomProposalResponse
    }
}
