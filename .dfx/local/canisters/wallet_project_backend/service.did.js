export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'check_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'receive_tokens' : IDL.Func([IDL.Text, IDL.Nat64], [IDL.Text], []),
    'send_tokens' : IDL.Func([IDL.Text, IDL.Nat64], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
