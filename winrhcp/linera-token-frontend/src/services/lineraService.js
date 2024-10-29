import axios from "axios";

const BASE_URL = "http://127.0.0.1:8080"; // Update to your backend URL

export const createToken = async (name, symbol, totalSupply) => {
    try {
        const response = await axios.post(`${BASE_URL}/create_token`, {
            name,
            symbol,
            total_supply: totalSupply,
        });
        return response.data;
    } catch (error) {
        console.error("Token creation error:", error);
        return { success: false, message: error.message };
    }
};
