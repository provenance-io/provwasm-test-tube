use test_tube_prov::{fn_execute, fn_query, Module, Runner};

use provwasm_std::types::cosmos::gov::v1::{
    QueryConstitutionRequest, QueryConstitutionResponse,
    QueryDepositRequest as QueryDepositRequestV1, QueryDepositResponse as QueryDepositResponseV1,
    QueryDepositsRequest as QueryDepositsRequestV1,
    QueryDepositsResponse as QueryDepositsResponseV1, QueryParamsRequest as QueryParamsRequestV1,
    QueryParamsResponse as QueryParamsResponseV1, QueryProposalRequest as QueryProposalRequestV1,
    QueryProposalResponse as QueryProposalResponseV1,
    QueryProposalsRequest as QueryProposalsRequestV1,
    QueryProposalsResponse as QueryProposalsResponseV1,
    QueryTallyResultRequest as QueryTallyResultRequestV1,
    QueryTallyResultResponse as QueryTallyResultResponseV1, QueryVoteRequest as QueryVoteRequestV1,
    QueryVoteResponse as QueryVoteResponseV1, QueryVotesRequest as QueryVotesRequestV1,
    QueryVotesResponse as QueryVotesResponseV1,
};
use provwasm_std::types::cosmos::gov::v1beta1::{
    MsgSubmitProposal, MsgSubmitProposalResponse, MsgVote, MsgVoteResponse, QueryDepositRequest,
    QueryDepositResponse, QueryDepositsRequest, QueryDepositsResponse, QueryParamsRequest,
    QueryParamsResponse, QueryProposalRequest, QueryProposalResponse, QueryProposalsRequest,
    QueryProposalsResponse, QueryTallyResultRequest, QueryTallyResultResponse, QueryVoteRequest,
    QueryVoteResponse, QueryVotesRequest, QueryVotesResponse,
};

pub struct Gov<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Gov<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Gov<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub submit_proposal: MsgSubmitProposal["/cosmos.gov.v1beta1.MsgSubmitProposal"] => MsgSubmitProposalResponse
    }

    fn_execute! {
        pub vote: MsgVote["/cosmos.gov.v1beta1.MsgVote"] => MsgVoteResponse
    }

    fn_query! {
        pub query_vote ["/cosmos.gov.v1beta1.Query/Vote"]: QueryVoteRequest => QueryVoteResponse
    }

    fn_query! {
        pub query_params ["/cosmos.gov.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_deposit ["/cosmos.gov.v1beta1.Query/Deposit"]: QueryDepositRequest => QueryDepositResponse
    }

    fn_query! {
        pub query_proposal ["/cosmos.gov.v1beta1.Query/Proposal"]: QueryProposalRequest => QueryProposalResponse
    }

    fn_query! {
        pub query_proposals ["/cosmos.gov.v1beta1.Query/Proposals"]: QueryProposalsRequest => QueryProposalsResponse
    }

    fn_query! {
        pub query_deposits ["/cosmos.gov.v1beta1.Query/Deposits"]: QueryDepositsRequest => QueryDepositsResponse
    }

    fn_query! {
        pub query_tally_result ["/cosmos.gov.v1beta1.Query/TallyResult"]: QueryTallyResultRequest => QueryTallyResultResponse
    }

    fn_query! {
        pub query_votes ["/cosmos.gov.v1beta1.Query/Votes"]: QueryVotesRequest => QueryVotesResponse
    }

    fn_query! {
        pub query_constitution ["/cosmos.gov.v1.Query/Constitution"]: QueryConstitutionRequest => QueryConstitutionResponse
    }

    fn_query! {
        pub query_proposal_v1 ["/cosmos.gov.v1.Query/Proposal"]: QueryProposalRequestV1 => QueryProposalResponseV1
    }

    fn_query! {
        pub query_proposals_v1 ["/cosmos.gov.v1.Query/Proposals"]: QueryProposalsRequestV1 => QueryProposalsResponseV1
    }

    fn_query! {
        pub query_vote_v1 ["/cosmos.gov.v1.Query/Vote"]: QueryVoteRequestV1 => QueryVoteResponseV1
    }

    fn_query! {
        pub query_votes_v1 ["/cosmos.gov.v1.Query/Votes"]: QueryVotesRequestV1 => QueryVotesResponseV1
    }

    fn_query! {
        pub query_params_v1 ["/cosmos.gov.v1.Query/Params"]: QueryParamsRequestV1 => QueryParamsResponseV1
    }

    fn_query! {
        pub query_deposit_v1 ["/cosmos.gov.v1.Query/Deposit"]: QueryDepositRequestV1 => QueryDepositResponseV1
    }

    fn_query! {
        pub query_deposits_v1 ["/cosmos.gov.v1.Query/Deposits"]: QueryDepositsRequestV1 => QueryDepositsResponseV1
    }

    fn_query! {
        pub query_tally_result_v1 ["/cosmos.gov.v1.Query/TallyResult"]: QueryTallyResultRequestV1 => QueryTallyResultResponseV1
    }
}
