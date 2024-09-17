import React from "react";

import ICON from "./ICON"
const CheckBox = ({toggleCompleted, index, todo}) => {
  return (
    <label class="container">
      <input 
      checked={todo.completed}
      type="checkbox"
      onChange={() => toggleCompleted(index)} />
      <ICON/>

    </label>
  );
  
};

export default CheckBox;
