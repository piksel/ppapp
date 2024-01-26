import { useRef, useState, useEffect, useMemo } from 'react'
import { io } from 'socket.io-client';
import { Message, User, Room, types, Vote, Round } from './types'

export const useSocket = () => {
    const [users, setUsers] = useState<User[]>([]);
    const [user, setUser] = useState<User>();
    const [userID, setUserID] = useState<string>();
    const [messages, setMessages] = useState<Message[]>([]);
    const [room, setRoom] = useState<Room>();
    const onceRef = useRef(false);

    const [votes, setVotes] = useState<Vote[]>([]);
    const [vote, setVote] = useState<Vote>();
    const [rounds, setRounds] = useState<Round[]>([]);


    const socket = useMemo(() => io(`${location.protocol.replace('http', 'ws')}//${location.host}`, { autoConnect: false }), []);

    useEffect(() => {
        if (onceRef.current) {
            return;
        }

        onceRef.current = true;

        const sessionID = localStorage.getItem("sessionID");

        socket.on("connect", () => {
            console.log("Connected to socket server");
            // console.log("joining room", currentRoom);

            // socket.emit("join", currentRoom);
        });

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
            msg.date = new Date(msg.date);
            setMessages((messages) => [...messages, msg]);
        });

        socket.on("room", (room) => {
            setRoom(room);
        });

        socket.on("vote", (vote) => {
            setVote(vote);
        });

        socket.on("votes", ({ votes }) => {
            setVotes(votes);
        });

        socket.on("rounds", ({ rounds }) => {
            setRounds(rounds);
        });

        socket.on("new round", () => {
            setVotes([]);
            setVote(undefined);
        });

        socket.on("user", (user) => {
            setUser(user);
        });

        socket.on("messages", (payload) => {
            const messages = payload.messages.map((msg: types.MessageDTO) => {
                return ({ ...msg, date: new Date(msg.date) });
            });
            setMessages(messages);
        });

        socket.on("users", payload => {
            const users: types.UserDTO[] = (payload && payload.users) ?? [];
            setUsers(users);
        });

        socket.onAny((event, ...args) => {
            console.info("[EVENT] %o: %O", event, args);
        })

        if (sessionID && sessionID != 'undefined') {
            socket.auth = { sessionID };
            socket.connect();
        }

    }, []);

    return { socket, messages, users, user, userID, room, votes, rounds, vote };
}