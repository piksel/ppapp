import { useRef, useState, useEffect, useMemo, useCallback } from "react";
import { Socket, io } from "socket.io-client";
import { Message, User, Room, Vote, Round, CurrentRound, ClientToServerEvents, ServerToClientEvents } from "./types";

export type ConnectionState = 'disconnected' | 'connecting' | 'connected';

export const useSocket = () => {
    const [users, setUsers] = useState<User[]>([]);
    const [user, setUser] = useState<User>();
    const [userID, setUserID] = useState<string>();
    const [messages, setMessages] = useState<Message[]>([]);
    const [room, setRoom] = useState<Room>();
    const [currentRoom, setCurrentRoom] = useState<string>();
    const [currentRound, setCurrentRound] = useState<CurrentRound>();
    const [state, setState] = useState<ConnectionState>('disconnected');
    const onceRef = useRef(false);

    const [votes, setVotes] = useState<Vote[]>([]);
    const [vote, setVote] = useState<Vote>();
    const [rounds, setRounds] = useState<Round[]>([]);

    const socket: Socket<ServerToClientEvents, ClientToServerEvents> = useMemo(
        () =>
            io(`${location.protocol.replace("http", "ws")}//${location.host}`, {
                autoConnect: false,
            }),
        [],
    );

    const tryAutoConnect = useCallback(() => {
        const sessionID = localStorage.getItem("sessionID");
        if (sessionID && sessionID != "undefined") {
            setState('connecting');
            socket.auth = { sessionID };
            socket.connect();
        }
    }, [socket]);

    useEffect(() => {
        if (state === 'disconnected') {
            console.log('Disconnected. Trying to reconnect in 5s...');
            const timeout = setTimeout(() => {
                tryAutoConnect()
            }, 5000);
            return () => clearTimeout(timeout);
        }
    }, [state, tryAutoConnect]);


    useEffect(() => {
        setVote(undefined);
    }, [rounds]);

    useEffect(() => {
        if (onceRef.current) {
            return;
        }

        onceRef.current = true;

        socket.on("connect", () => {
            console.log("Connected to socket server");
            if (currentRoom) {
                console.log("Re-joining room %o", currentRoom);
                socket.emit("join", currentRoom, r => {
                    if (r.type === 'Error') {
                        console.error('Failed to auto-join room %o: ', currentRoom, r.error);
                    } else {
                        console.error('Auto-joined room %o: ', currentRoom);
                    }
                });
            }
        });

        socket.on('disconnect', () => {
            setState('disconnected');
        })

        socket.on("session", (data) => {
            const { sessionID, userID } = data;
            // attach the session ID to the next reconnection attempts
            socket.auth = { sessionID };
            // store it in the localStorage
            localStorage.setItem("sessionID", sessionID);
            // save the ID of the user
            setUserID(userID);
            // socket.userID = userID;
        });

        socket.on("message", (msg) => {
            const message = { ...msg, date: new Date(msg.date) };
            setMessages((messages) => [...messages, message]);
        });

        socket.on("room", (room) => {
            setRoom(room);
            setCurrentRoom(room.roomID);
        });

        socket.on("vote", (vote) => {
            setVote(vote);
        });

        socket.on("votes", (votes) => {
            setVotes(votes);
        });

        socket.on("rounds", (rounds) => {
            setRounds(rounds);
        });

        socket.on("current round", (currentRound) => {
            setCurrentRound(currentRound);
        });

        socket.on("user", (user) => {
            setUser(user);
        });


        socket.on("user updated", (updated: User) => {
            setUsers(users => users.map(u => u.userID === updated.userID ? updated : u));
        });

        socket.on("messages", (msgs) => {
            const messages = msgs.map((msg) => {
                return { ...msg, date: new Date(msg.date) };
            });
            setMessages(messages);
        });

        socket.on("users", (users) => {
            setUsers(users);
        });

        socket.onAny((event, ...args) => {
            console.info("[EVENT] %o: %O", event, args);
        });

        tryAutoConnect();

    }, []);

    return { socket, messages, users, user, userID, room, votes, rounds, vote, currentRound, state };
};
