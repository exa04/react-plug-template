import "./styles.css";

import { usePluginContext } from "./bindings/PluginProvider";

function App() {
  const ctx = usePluginContext();
  const gain = ctx.parameters.gain;

  return (
    <div className="container">
      <div>
        <h1>{{ plugin_name }}</h1>
        <p>{{ description }}</p>
      </div>
      <hr />
      <div className="input-group">
        <div className="labeled-input">
          <div>
            {gain.name}: {gain.value_to_string(gain.value)}
            {gain.unit}
          </div>
          <input
            type="range"
            className="slider"
            min={0}
            max={1}
            step={0.001}
            value={gain.normalizedValue}
            onChange={(e) => {
              gain.setNormalizedValue(parseFloat(e.target.value));
            }}
          />
        </div>
        <button onClick={() => gain.resetValue()}>Reset</button>
      </div>
    </div>
  );
}

export default App;
