import {useState, useRef, useEffect} from "react";
import {io} from "socket.io-client";
import {Bars3Icon, } from "@heroicons/react/24/outline";
import {Sidebar} from "./components/sidebar/normal/Sidebar.jsx";
import {RoomMsgsList} from "./components/RoomMsgsList.jsx";
import {MsgSubmitBox} from "./components/MsgSubmitBox.jsx";
import {rooms} from "./utils/rooms.js";
import {TransitiveSidebar} from "./components/sidebar/transitive/TransitiveSidebar.jsx";

function App() {
  const [users, setUsers] = useState([]);
  const [user, setUser] = useState({});
  const [messages, setMessages] = useState([]);
  const [currentRoom, setCurrentRoom] = useState(rooms[0]);
  const [socket, setSocket] = useState(null);
  const onceRef = useRef(false);
  const [sidebarOpen, setSidebarOpen] = useState(false);

  useEffect(() => {
    setMessages([]);
    socket?.emit("join", currentRoom);
  }, [currentRoom, socket]);

  useEffect(() => {
    if (onceRef.current) {
      return;
    }

    onceRef.current = true;

    const socket = io("ws://localhost:3000", { autoConnect: false });
    setSocket(socket);

    const sessionID = localStorage.getItem("sessionID");

    socket.on("connect", () => {
      console.log("Connected to socket server");
      console.log("joining room", currentRoom);

      socket.emit("join", currentRoom);
    });

    socket.on("session", (data) => {
      const { sessionID, userID } = data;
      // attach the session ID to the next reconnection attempts
      socket.auth = { sessionID };
      // store it in the localStorage
      localStorage.setItem("sessionID", sessionID);
      // save the ID of the user
      socket.userID = userID;
    });

    socket.on("message", (msg) => {
      msg.date = new Date(msg.date);
      setMessages((messages) => [...messages, msg]);
    });

    socket.on("messages", (payload) => {
      const messages = payload.messages.map((msg) => {
        msg.date = new Date(msg.date);
        return msg;
      });
      setMessages(messages);
    });

    socket.on("users", payload => {
      const users = (payload && payload.users) ?? []; 
      setUsers(users);
      const user = users.find(u => u.userID == socket.userID);
      setUser(user);
    });

    socket.onAny((event, ...args) => {
      console.info("[EVENT] %o: %O", event, args);
    })

    if (sessionID && sessionID != 'undefined') {
      socket.auth = {sessionID};
      socket.connect(); 
    }

  }, []);

  return (
    <>
      <main className="h-screen w-screen flex text-ctp-text">
        {socket && socket.connected ? (<>
        <TransitiveSidebar
          sidebarOpen={sidebarOpen}
          setSidebarOpen={setSidebarOpen}
          currentRoom={currentRoom}
          setCurrentRoom={setCurrentRoom}
        />
        <Sidebar
          currentRoom={currentRoom}
          setCurrentRoom={setCurrentRoom}
        />
        <div className="h-screen p-4 bg-ctp-crust flex flex-col flex-grow justify-end">
          <div className="bg-ctp-base rounded-t-lg flex-grow">
            <div className="sticky top-0 z-40 flex items-center gap-x-6 bg-ctp-mantle px-2 sm:px-6 lg:hidden">
              <button
                type="button"
                className="-m-2.5 p-2.5 text-gray-400 lg:hidden"
                onClick={() => setSidebarOpen(true)}
              >
                <span className="sr-only">Open sidebar</span>
                <Bars3Icon className="h-6 w-6" aria-hidden="true"/>
              </button>
              <div className="flex-1 text-sm font-semibold leading-6 text-white">
                <h1 className="text-2xl text-white font-bold py-4">
                  {currentRoom}
                </h1>
                
              </div>
            </div>

            <h1 className="hidden lg:block text-2xl text-center text-white font-bold my-4">
              {currentRoom}
            </h1>
            <RoomMsgsList messages={messages} users={users} />
          </div>
          <MsgSubmitBox socket={socket} currentRoom={currentRoom}/>
        </div>
        </>) : (
          <div>
            <Login socket={socket} />
          </div>
        )}
      </main>
    </>
  );
}

const Login = (props) => {
  const {socket} = props;
  const [username, setUsername] = useState("");

  const connect = () => {
    socket.auth = {username};
    socket.connect();
  }

  return (
    <div className="bg-ctp-crust p-4">            
    <input 
    className="flex-1 p-2 rounded-l-md bg-ctp-text text-ctp-base placeholder-ctp-subtext0"
    placeholder="Username"
    type="text" value={username} onChange={e => setUsername(e.target.value)} />
    <button 
    className="bg-ctp-blue px-6 font-bold text-ctp-base p-2 rounded-r-md"
    onClickCapture={() => connect()}>Connect</button>
    </div>
  )
}

export default App;
