"use client";
import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/api/notification";

const Home = () => {
  const [input, setInput] = useState("");
  const [mainTopic, setMainTopic] = useState("");
  const [permissionGranted, setPermissionGranted] = useState(false);

  // アプリを起動した時に、通知の許可設定がされているかどうかの確認
  useEffect(() => {
    const checkNotificationPermission = async () => {
      // 非同期処理で、権限の確認
      let permissionGranted = await isPermissionGranted();

      // 権限が許可されていなかった時の処理
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === "granted";
      }

      // 権限が許可されていれば、useStateをtrueに変更
      setPermissionGranted(permissionGranted);
    };

    checkNotificationPermission();
  }, []);

  useEffect(() => {
    const initializeSystemPrompt = async () => {
      try {
        await invoke("set_system_prompt", { mainTopic: "デフォルトの議題" });
      } catch (error) {
        console.error("Failed to set system prompt:", error);
      }
    };

    initializeSystemPrompt();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const res = await invoke("generate_response", { input });
      if (res !== "deviation") return;

      if (!permissionGranted) {
        return;
      }

      sendNotification({
        title: "お知らせ",
        body: "設定された話題内容から脱線しています！",
      });
    } catch (error) {
      console.error(error);
    }
  };

  const handleReset = () => {
    setInput("");
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-md">
      <h1 className="text-2xl font-bold mb-4 text-center text-gray-800">
        シンプル入出力UI
      </h1>
      <div className="mb-4">
        <input
          type="text"
          value={mainTopic}
          onChange={(e) => setMainTopic(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="議題を入力してください"
        />
        <button
          onClick={async () => {
            try {
              await invoke("set_system_prompt", { mainTopic });
              alert("議題が設定されました");
            } catch (error) {
              console.error("Failed to set system prompt:", error);
              alert("議題の設定に失敗しました");
            }
          }}
          className="mt-2 w-full bg-green-500 text-white py-2 px-4 rounded-md hover:bg-green-600 transition duration-300"
        >
          議題を設定
        </button>
      </div>
      <form onSubmit={handleSubmit} className="mb-4">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="ここに入力してください"
        />
        <button
          type="button"
          onClick={handleReset}
          className="mt-2 w-full bg-gray-500 text-white py-2 px-4 rounded-md hover:bg-gray-600 transition duration-300"
        >
          リセット
        </button>
        <button
          type="submit"
          className="mt-2 w-full bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-600 transition duration-300"
        >
          送信
        </button>
      </form>
    </div>
  );
};

export default Home;
