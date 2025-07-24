mod util;

use bootstrap_common::{SessionMemberLocation};
use tokio::test;

use crate::util::TestBootstrapServer;

#[test]
async fn test_all() -> anyhow::Result<()> {
    let server = TestBootstrapServer::new().await?;

    let mem1 = SessionMemberLocation {
        addr: "::1".parse()?,
        port: 0
    };

    let session_id = server.client().create_session().await?;

    assert_eq!(0, server.client().get_session(&session_id).await?.len());

    server.client().update_session(&session_id, &mem1).await?;

    assert!(server.client().get_session(&session_id).await?.contains(&mem1));

    server.client().update_session(&session_id, &mem1).await?;

    assert_eq!(1, server.client().get_session(&session_id).await?.len());

    let mem2 = SessionMemberLocation {
        addr: "::2".parse()?,
        port: 0
    };

    server.client().update_session(&session_id, &mem2).await?;

    let session = server.client().get_session(&session_id).await?;
    
    assert!(session.contains(&mem1));
    assert!(session.contains(&mem2));
    assert_eq!(2, session.len());

    Ok(())
}