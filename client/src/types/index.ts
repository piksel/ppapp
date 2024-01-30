import * as types from './ppapi';
// export * from './ppapi'

export type Message = Omit<types.MessageDTO, 'date'> & { date: Date };
export type User = types.UserDTO;
export type Room = types.RoomDTO;
export type Vote = types.VoteDTO;
export type Round = types.RoundDTO;
export type CurrentRound = types.CurrentRoundDTO;
export type Result = types.AckResult;

export interface Session {
    userID: string;
    sessionID: string;
}

export interface ServerToClientEvents {
    noArg: () => void;
    user: (user: User) => void;
    vote: (vote: Vote) => void;
    room: (room: Room) => void;
    session: (session: Session) => void;
    message: (message: types.MessageDTO) => void;
    users: (users: User[]) => void;
    rounds: (rounds: Round[]) => void;
    votes: (votes: Vote[]) => void;
    messages: (messages: types.MessageDTO[]) => void;
    ['current round']: (currentRound: CurrentRound) => void;
    ['user updated']: (user: User) => void;

}

export interface ClientToServerEvents {
    ['vote']: (score: string, callback: (r: Result) => void) => void;
    ['join']: (roomId: string, callback: (r: Result) => void) => void;
    ['create room']: (roomName: string, callback: (r: Result) => void) => void;
    ['update user']: (user: User, callback: (r: Result) => void) => void;
    ['end vote']: (roomId: string, callback: (r: Result) => void) => void;
    ['new round']: (roomId: string, callback: (r: Result) => void) => void;
}