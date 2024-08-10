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
            {gain.name}: {gain.value}
            {gain.unit}
          </div>
          <input
            type="range"
            className="slider"
            min={0}
            max={1}
            step={0.001}
            value={gain.range.normalize(gain.rawValue)}
            onChange={(e) => {
              gain.setValue(gain.range.unnormalize(Number(e.target.value)));
            }}
          />
        </div>
        <button onClick={() => gain.setValue(gain.defaultValue)}>Reset</button>
      </div>
    </div>
  );
}

export default App;
