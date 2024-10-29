import React, { useState } from "react";
import { createToken } from "../services/lineraService";

function CreateToken() {
    const [name, setName] = useState("");
    const [symbol, setSymbol] = useState("");
    const [totalSupply, setTotalSupply] = useState("");
    const [message, setMessage] = useState("");

    const handleCreateToken = async () => {
        const result = await createToken(name, symbol, parseInt(totalSupply));
        setMessage(result ? "Token created successfully!" : "Error creating token.");
    };

    return (
        <div>
            <h2>Create Token</h2>
            <input
                type="text"
                placeholder="Token Name"
                value={name}
                onChange={(e) => setName(e.target.value)}
            />
            <input
                type="text"
                placeholder="Symbol"
                value={symbol}
                onChange={(e) => setSymbol(e.target.value)}
            />
            <input
                type="number"
                placeholder="Total Supply"
                value={totalSupply}
                onChange={(e) => setTotalSupply(e.target.value)}
            />
            <button onClick={handleCreateToken}>Create Token</button>
            <p>{message}</p>
        </div>
    );
}

export default CreateToken;
