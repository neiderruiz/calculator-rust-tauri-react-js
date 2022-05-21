import { useEffect, useState } from "react";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { actions } from "./data/actions";

function App() {
  const [valueCalculator, setValueCalculator] = useState<String>("");
  const [newValue, setNewValue] = useState({
    value: "",
    time: 0,
  });

  useEffect(() => {
    appWindow.setSize(new LogicalSize(500, 462));
    window.addEventListener("keydown", (e) => {
      setNewValue({
        value: e.key,
        time: new Date().getTime(),
      });
    });
  }, []);

  useEffect(() => {
    invoke("select_operation", {
      value: `${newValue.value}`,
      valueInput: valueCalculator,
    }).then((response) => {
      if (typeof response === "string") {
        console.log(response.length);
        setValueCalculator(response);
      }
    });
  }, [newValue]);

  return (
    <div className="container-calculator ">
      <h1>Calculadora</h1>
      <div className="container-input">
        <input
          id="insert"
          type="text"
          value={valueCalculator.toString()}
          placeholder="Ingresar operacion"
        />
      </div>
      {valueCalculator.length}
      <div id="container-symbols">
        {actions.map((action) => (
          <div
            className="item"
            data-action="${action.value}"
            onClick={() => {
              invoke("select_operation", {
                value: `${action.value}`,
                valueInput: valueCalculator,
              }).then((response) => {
                if (typeof response === "string") {
                  setValueCalculator(response);
                }
              });
            }}
          >
            {action.name}
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
