"use client";
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const Home = () => {
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    invoke("generate_response", { input })
      .then((res) => {
        // resの内容をoutputに設定
        setOutput(JSON.stringify(res));
      })
      .catch((e) => console.error(e));
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-md">
      <h1 className="text-2xl font-bold mb-4 text-center text-gray-800">
        シンプル入出力UI
      </h1>

      <form onSubmit={handleSubmit} className="mb-4">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="ここに入力してください"
        />
        <button
          type="submit"
          className="mt-2 w-full bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-600 transition duration-300"
        >
          送信
        </button>
      </form>

      <div className="mt-6">
        <h2 className="text-lg font-semibold mb-2 text-gray-700">出力:</h2>
        <div className="p-3 bg-gray-100 rounded-md min-h-[50px]">
          {output || "出力がここに表示されます"}
        </div>
      </div>
    </div>
  );
};

export default Home;
