import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'check_balance' : ActorMethod<[], bigint>,
  'receive_tokens' : ActorMethod<[string, bigint], string>,
  'send_tokens' : ActorMethod<[string, bigint], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
