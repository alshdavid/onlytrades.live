// let conn = CTraderConnection::connect(CTraderConnectionOptions { live: false }).await?;

//   let ctrader_access_token = std::env::var("CTRADER_ACCESS_TOKEN")?;
//   let ctrader_account_id: i64 = std::env::var("CTRADER_ACCOUNT_ID")?.parse()?;

//   let mut rx = conn.subscribe().await;

//   conn
//     .send(CTraderRequestType::ApplicationAuthReq(
//       ApplicationAuthReq {
//         client_msg_id: None,
//         client_id: env.ctrader_client_id.clone(),
//         client_secret: env.ctrader_client_secret.clone(),
//       },
//     ))
//     .await?;
//   let _ = rx.recv().await;

//   conn
//     .send(CTraderRequestType::AccountAuthReq(AccountAuthReq {
//       client_msg_id: None,
//       ctid_trader_account_id: ctrader_account_id.clone(),
//       access_token: ctrader_access_token.clone(),
//     }))
//     .await?;
//   let _ = rx.recv().await;
//   drop(rx);

//   let mut process = IpcTcpInstance::new(GenericProcessOptions {
//     network: true,
//     read_only_fs: false,
//     memory: 1024 * 32,
//     cpus: 100,
//     volumes: HashMap::new(),
//     env: HashMap::from_iter(vec![("ACCOUNT_ID".to_string(), ctrader_account_id.to_string())]),
//     sandbox: SandboxType::None {
//       command: vec![
//         "/mnt/data/Development/alshdavid/onlytrades.live/target/debug/onlytrades_examples_rust_bot"
//           .to_string(),
//       ],
//     },
//   })
//   .await?;

//   tokio::spawn({
//     let tx = process.writer();
//     let mut rx = conn.subscribe().await;
//     async move {
//       while let Some(res) = rx.recv().await {
//         let message = serde_json::to_vec(&res).unwrap();
//         let _ = tx.send(message);
//       }
//     }
//   });


//   tokio::spawn({
//     let mut rx = process.reader()?;
//     async move {
//       while let Some(res) = rx.recv().await {
//         let message = serde_json::from_slice::<CTraderRequestType>(&res).unwrap();
//         dbg!(&message);
//         conn.send(message).await.unwrap();
//       }
//     }
//   });


//   let mut stdout = process.stdout().await;
//   tokio::spawn(async move {
//     while let Some(line) = stdout.recv().await {
//       println!("STDOUT: {}", line)
//     }
//   });

//   let mut stderr = process.stderr().await;
//   tokio::spawn(async move {
//     while let Some(line) = stderr.recv().await {
//       println!("STDERR: {}", line)
//     }
//   });

//   println!("MAIN: WAIT");
//   tokio::time::sleep(Duration::from_secs(5)).await;
//   println!("MAIN: DONE");






  // let ctrader_tokens = ctrader_token_repository
  //   .get_tokens_for_profile(&bot.profile_id)
  //   .await?
  //   .unwrap();

  // let account = ctrader_account_repository
  //   .find_by_profile_id(&bot.profile_id)
  //   .await
  //   .unwrap()
  //   .into_iter()
  //   .find(|a| a.account_id == deployment.account_id)
  //   .unwrap();

  // let ctrader_access_token = std::env::var("CTRADER_ACCESS_TOKEN")?;
  // let ctrader_account_id: i64 = std::env::var("CTRADER_ACCOUNT_ID")?.parse()?;

  // let conn = CTraderConnection::connect(CTraderConnectionOptions { live: false })
  //   .await
  //   .unwrap();

  // let mut rx = conn.subscribe().await;

  // conn
  //   .send(CTraderRequestType::ApplicationAuthReq(ApplicationAuthReq {
  //     client_msg_id: None,
  //     client_id: env.ctrader_client_id.clone(),
  //     client_secret: env.ctrader_client_secret.clone(),
  //   }))
  //   .await?;

  // rx.recv().await.unwrap().unwrap();

  // conn
  //   .send(CTraderRequestType::AccountAuthReq(AccountAuthReq {
  //     client_msg_id: None,
  //     ctid_trader_account_id: ctrader_account_id.clone(),
  //     access_token: ctrader_access_token.clone(),
  //   }))
  //   .await?;

  // rx.recv().await.unwrap().unwrap();
  // drop(rx);




  // let code = include_bytes!("/mnt/data/Development/alshdavid/onlytrades.live/target/debug/onlytrades_examples_rust_bot");


  // let temp_file = create_temp_file(&tmp.path(), "test", code)
  //   .await
  //   .unwrap();


  // tokio::time::sleep(Duration::from_secs(10)).await;
