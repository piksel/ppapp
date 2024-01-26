import types from './ppapi'
export * from './ppapi'

export type Message = Omit<types.MessageDTO, 'date'> & { date: Date };
export type User = types.UserDTO;
export type Room = types.RoomDTO;
export type Vote = types.VoteDTO;
export type Round = types.RoundDTO;