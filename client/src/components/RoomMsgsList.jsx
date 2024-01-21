import { useMemo } from "react";
import {classNames} from "../utils/class-names.js";
import {colorForName} from "../utils/color-for-name.js";

export function RoomMsgsList(props) {

  const users = useMemo(() => {
    if(!Array.isArray(props.users)) return {};
    return Object.fromEntries(props.users.map(u => [u.userID, u]))
  }, [props.users]);

  const messages = useMemo(() => props.messages.map(m => ({...m, user: users[m.from] ?? {username: m.from} })), [
    users,
    props.messages
  ]);
  
  return (
    <ul className="p-4">
      {messages?.map((msg, index) => (
        <li
          key={index}
          className="flex w-full justify-start gap-x-4 mb-4 align-top"
        >
          <div>
            <div className="flex flex-row gap-x-6 items-center">
              <p
                className={classNames(
                  "text-sm font-semibold",
                  `text-${colorForName(msg.user.username ?? '')}`,
                )}
              >
                {msg.user.username}
              </p>
              <p className="text-ctp-text text-sm">
                {msg.date.toLocaleString('sv-se')}
              </p>
            </div>
            <p className="text-ctp-text mt-1 text-lg">{msg.content}</p>
          </div>
        </li>
      ))}
    </ul>
  )
}
