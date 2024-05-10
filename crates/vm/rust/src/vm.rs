use {
    crate::{ApiProvider, ContractWrapper, VmError, VmResult, CONTRACTS},
    cw_app::{PrefixStore, QueryProvider, Vm},
    cw_types::{from_json_slice, to_json_vec, Context},
};

pub struct RustVm {
    storage: PrefixStore,
    querier: QueryProvider<Self>,
    program: ContractWrapper,
}

impl Vm for RustVm {
    type Error = VmError;
    type Program = ContractWrapper;

    fn build_instance(
        storage: PrefixStore,
        querier: QueryProvider<Self>,
        program: Self::Program,
    ) -> VmResult<Self> {
        Ok(Self {
            storage,
            querier,
            program,
        })
    }

    fn call_in_0_out_1(mut self, name: &str, ctx: &Context) -> VmResult<Vec<u8>> {
        let cell = CONTRACTS; // unclear why this is necessary?
        let contract = &cell.get().unwrap()[self.program.index];
        let out = match name {
            "receive" => {
                let res = contract
                    .receive(ctx.clone(), &mut self.storage, &ApiProvider, &self.querier)
                    .into_std_result()?; // TODO: it doesn't really make sense to cast this to a StdError here. just a quick hack
                to_json_vec(&res)?
            },
            _ => {
                return Err(VmError::IncorrectNumberOfInputs {
                    name: name.into(),
                    num: 0,
                })
            },
        };
        Ok(out)
    }

    fn call_in_1_out_1(
        mut self,
        name: &str,
        ctx: &Context,
        param1: impl AsRef<[u8]>,
    ) -> VmResult<Vec<u8>> {
        let cell = CONTRACTS; // unclear why this is necessary?
        let contract = &cell.get().unwrap()[self.program.index];
        let out = match name {
            "instantiate" => {
                let msg = from_json_slice(param1)?;
                let res = contract
                    .instantiate(ctx.clone(), &mut self.storage, &ApiProvider, &self.querier, msg)
                    .into_std_result()?;
                to_json_vec(&res)?
            },
            "execute" => {
                let msg = from_json_slice(param1)?;
                let res = contract
                    .execute(ctx.clone(), &mut self.storage, &ApiProvider, &self.querier, msg)
                    .into_std_result()?;
                to_json_vec(&res)?
            },
            "migrate" => {
                let msg = from_json_slice(param1)?;
                let res = contract
                    .migrate(ctx.clone(), &mut self.storage, &ApiProvider, &self.querier, msg)
                    .into_std_result()?;
                to_json_vec(&res)?
            },
            "query" => {
                let msg = from_json_slice(param1)?;
                let res = contract
                    .query(ctx.clone(), &self.storage, &ApiProvider, &self.querier, msg)
                    .into_std_result()?;
                to_json_vec(&res)?
            },
            _ => {
                return Err(VmError::IncorrectNumberOfInputs {
                    name: name.into(),
                    num: 1,
                })
            },
        };
        Ok(out)
    }

    fn call_in_2_out_1(
        mut self,
        name: &str,
        ctx: &Context,
        param1: impl AsRef<[u8]>,
        param2: impl AsRef<[u8]>,
    ) -> VmResult<Vec<u8>> {
        let cell = CONTRACTS; // unclear why this is necessary?
        let contract = &cell.get().unwrap()[self.program.index];
        let out = match name {
            "reply" => {
                let msg = from_json_slice(param1)?;
                let submsg_res = from_json_slice(param2)?;
                let res = contract
                    .reply(
                        ctx.clone(),
                        &mut self.storage,
                        &ApiProvider,
                        &self.querier,
                        msg,
                        submsg_res,
                    )
                    .into_std_result()?;
                to_json_vec(&res)?
            },
            _ => {
                return Err(VmError::IncorrectNumberOfInputs {
                    name: name.into(),
                    num: 2,
                })
            },
        };
        Ok(out)
    }
}
