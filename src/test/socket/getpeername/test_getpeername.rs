/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */

use test_utils::AsMutPtr;
use test_utils::TestEnvironment as TestEnv;

struct GetpeernameArguments {
    fd: libc::c_int,
    addr: Option<libc::sockaddr_in>, // if None, a null pointer should be used
    addr_len: Option<libc::socklen_t>, // if None, a null pointer should be used
}

fn main() -> Result<(), String> {
    // should we restrict the tests we run?
    let filter_shadow_passing = std::env::args().any(|x| x == "--shadow-passing");
    let filter_libc_passing = std::env::args().any(|x| x == "--libc-passing");
    // should we summarize the results rather than exit on a failed test
    let summarize = std::env::args().any(|x| x == "--summarize");

    let mut tests = get_tests();
    if filter_shadow_passing {
        tests = tests
            .into_iter()
            .filter(|x| x.passing(TestEnv::Shadow))
            .collect()
    }
    if filter_libc_passing {
        tests = tests
            .into_iter()
            .filter(|x| x.passing(TestEnv::Libc))
            .collect()
    }

    test_utils::run_tests(&tests, summarize)?;

    println!("Success.");
    Ok(())
}

fn get_tests() -> Vec<test_utils::ShadowTest<(), String>> {
    let tests: Vec<test_utils::ShadowTest<_, _>> = vec![
        test_utils::ShadowTest::new(
            "test_invalid_fd",
            test_invalid_fd,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_non_existent_fd",
            test_non_existent_fd,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_non_socket_fd",
            test_non_socket_fd,
            [TestEnv::Libc].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_non_connected_fd",
            test_non_connected_fd,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_null_addr",
            test_null_addr,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_null_len",
            test_null_len,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_short_len",
            test_short_len,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_zero_len",
            test_zero_len,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_listening_socket",
            test_listening_socket,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_after_close",
            test_after_close,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_unbound_dgram_socket",
            test_unbound_dgram_socket,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_bound_dgram_socket",
            test_bound_dgram_socket,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_connected_dgram_socket",
            test_connected_dgram_socket,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_connected_before_accepted",
            test_connected_before_accepted,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_connected_socket",
            test_connected_socket,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_accepted_socket",
            test_accepted_socket,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
        test_utils::ShadowTest::new(
            "test_sockname_peername",
            test_sockname_peername,
            [TestEnv::Libc, TestEnv::Shadow].iter().cloned().collect(),
        ),
    ];

    tests
}

/// Test getpeername using an argument that cannot be a fd.
fn test_invalid_fd() -> Result<(), String> {
    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: -1,
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    check_getpeername_call(&mut args, Some(libc::EBADF))
}

/// Test getpeername using an argument that could be a fd, but is not.
fn test_non_existent_fd() -> Result<(), String> {
    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: 8934,
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    check_getpeername_call(&mut args, Some(libc::EBADF))
}

/// Test getpeername using a valid fd that is not a socket.
fn test_non_socket_fd() -> Result<(), String> {
    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: 0, // assume the fd 0 is already open and is not a socket
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    check_getpeername_call(&mut args, Some(libc::ENOTSOCK))
}

/// Test getpeername using a valid fd, but that is not connected to a peer.
fn test_non_connected_fd() -> Result<(), String> {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd >= 0);

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd,
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    test_utils::run_and_close_fds(&[fd], || {
        check_getpeername_call(&mut args, Some(libc::ENOTCONN))
    })
}

/// A helper function to start a server on one fd and connect another fd to it. Returns the accepted fd.
fn connect_helper(fd_client: libc::c_int, fd_server: libc::c_int) -> libc::c_int {
    // the server address
    let mut server_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: 0u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: libc::INADDR_LOOPBACK.to_be(),
        },
        sin_zero: [0; 8],
    };

    // bind on the server address
    {
        let rv = unsafe {
            libc::bind(
                fd_server,
                &server_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                std::mem::size_of_val(&server_addr) as u32,
            )
        };
        assert_eq!(rv, 0);
    }

    // get the assigned port number
    {
        let mut server_addr_size = std::mem::size_of_val(&server_addr) as u32;
        let rv = unsafe {
            libc::getsockname(
                fd_server,
                &mut server_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                &mut server_addr_size as *mut libc::socklen_t,
            )
        };
        assert_eq!(rv, 0);
        assert_eq!(server_addr_size, std::mem::size_of_val(&server_addr) as u32);
    }

    // listen for connections
    {
        let rv = unsafe { libc::listen(fd_server, 10) };
        assert_eq!(rv, 0);
    }

    // connect to the server address
    {
        let rv = unsafe {
            libc::connect(
                fd_client,
                &server_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                std::mem::size_of_val(&server_addr) as u32,
            )
        };
        assert_eq!(rv, 0);
    }

    // shadow needs to run events, otherwise the accept call won't know it
    // has an incoming connection (SYN packet)
    {
        let rv = unsafe { libc::usleep(10000) };
        assert_eq!(rv, 0);
    }

    // accept the connection
    let fd = unsafe { libc::accept(fd_server, std::ptr::null_mut(), std::ptr::null_mut()) };
    assert!(fd >= 0);

    fd
}

/// Test getpeername using a valid fd, but with a NULL address.
fn test_null_addr() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd_client,
        addr: None,
        addr_len: Some(5),
    };

    test_utils::run_and_close_fds(&[fd_server, fd_client, fd_accepted], || {
        check_getpeername_call(&mut args, Some(libc::EFAULT))
    })
}

/// Test getpeername using a valid fd and address, a NULL address length.
fn test_null_len() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd_client,
        addr: Some(addr),
        addr_len: None,
    };

    test_utils::run_and_close_fds(&[fd_server, fd_client, fd_accepted], || {
        check_getpeername_call(&mut args, Some(libc::EFAULT))
    })
}

/// Test getpeername using a valid fd and address, but an address length that is too small.
fn test_short_len() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // the sockaddr that we expect to have after calling getpeername()
    let expected_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        // we don't know the port
        sin_port: 0u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: libc::INADDR_LOOPBACK.to_be(),
        },
        // since our buffer will be short by one byte, we will only be missing one byte of sin_zero
        sin_zero: [0, 0, 0, 0, 0, 0, 0, 1],
    };

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd_client,
        addr: Some(addr),
        addr_len: Some((std::mem::size_of_val(&addr) - 1) as u32),
    };

    // if the buffer was too small, the returned data will be truncated but we won't get an error
    test_utils::run_and_close_fds(&[fd_server, fd_client, fd_accepted], || {
        check_getpeername_call(&mut args, None)
    })?;

    // check that the returned length is expected
    test_utils::result_assert_eq(
        args.addr_len.unwrap() as usize,
        std::mem::size_of_val(&addr),
        "Unexpected addr length",
    )?;

    // check that the returned address is expected
    sockaddr_check_equal(
        &args.addr.unwrap(),
        &expected_addr,
        /* ignore_port= */ true,
    )?;

    // check that the port is valid
    test_utils::result_assert(args.addr.unwrap().sin_port > 0, "Unexpected port")
}

/// Test getpeername using a valid fd and address, but an address length of 0.
fn test_zero_len() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // the sockaddr that we expect to have after calling getpeername();
    let expected_addr = addr;

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd_client,
        addr: Some(addr),
        addr_len: Some(0u32),
    };

    // if the buffer was too small, the returned data will be truncated but we won't get an error
    test_utils::run_and_close_fds(&[fd_server, fd_client, fd_accepted], || {
        check_getpeername_call(&mut args, None)
    })?;

    // check that the returned length is expected
    test_utils::result_assert_eq(
        args.addr_len.unwrap() as usize,
        std::mem::size_of_val(&addr),
        "Unexpected addr length",
    )?;

    // check that the returned address is expected
    sockaddr_check_equal(
        &args.addr.unwrap(),
        &expected_addr,
        /* ignore_port= */ false,
    )
}

/// Test getpeername on a listening socket.
fn test_listening_socket() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd_server,
        addr: Some(addr),
        addr_len: Some((std::mem::size_of_val(&addr) - 1) as u32),
    };

    test_utils::run_and_close_fds(&[fd_server, fd_client, fd_accepted], || {
        check_getpeername_call(&mut args, Some(libc::ENOTCONN))
    })
}

/// Test getpeername after closing the socket.
fn test_after_close() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // close the socket
    {
        let rv = unsafe { libc::close(fd_client) };
        assert_eq!(rv, 0);
    }

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd_client,
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    test_utils::run_and_close_fds(&[fd_server, fd_accepted], || {
        check_getpeername_call(&mut args, Some(libc::EBADF))
    })
}

/// Test getpeername using an unbound datagram socket.
fn test_unbound_dgram_socket() -> Result<(), String> {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };
    assert!(fd >= 0);

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd,
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    test_utils::run_and_close_fds(&[fd], || {
        check_getpeername_call(&mut args, Some(libc::ENOTCONN))
    })
}

/// Test getpeername using a bound datagram socket.
fn test_bound_dgram_socket() -> Result<(), String> {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };
    assert!(fd >= 0);

    // bind on some address
    {
        let bind_addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port: 0u16.to_be(),
            sin_addr: libc::in_addr {
                s_addr: libc::INADDR_LOOPBACK.to_be(),
            },
            sin_zero: [0; 8],
        };

        let rv = unsafe {
            libc::bind(
                fd,
                &bind_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                std::mem::size_of_val(&bind_addr) as u32,
            )
        };
        assert_eq!(rv, 0);
    }

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd,
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    test_utils::run_and_close_fds(&[fd], || {
        check_getpeername_call(&mut args, Some(libc::ENOTCONN))
    })
}

/// Test getpeername using a "connected" datagram socket.
fn test_connected_dgram_socket() -> Result<(), String> {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };
    assert!(fd >= 0);

    // some server address
    let bind_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: 11111u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: libc::INADDR_LOOPBACK.to_be(),
        },
        sin_zero: [0; 8],
    };

    // connect to the address
    {
        let rv = unsafe {
            libc::connect(
                fd,
                &bind_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                std::mem::size_of_val(&bind_addr) as u32,
            )
        };
        assert_eq!(rv, 0);
    }

    // the sockaddr that we expect to have after calling getpeername()
    let expected_addr = bind_addr;

    // fill the sockaddr with dummy data
    let addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    // getpeername() may mutate addr and addr_len
    let mut args = GetpeernameArguments {
        fd: fd,
        addr: Some(addr),
        addr_len: Some(std::mem::size_of_val(&addr) as u32),
    };

    test_utils::run_and_close_fds(&[fd], || check_getpeername_call(&mut args, None))?;

    // check that the returned length is expected
    test_utils::result_assert_eq(
        args.addr_len.unwrap() as usize,
        std::mem::size_of_val(&addr),
        "Unexpected addr length",
    )?;

    // check that the returned address is expected
    sockaddr_check_equal(
        &args.addr.unwrap(),
        &expected_addr,
        /* ignore_port= */ false,
    )
}

/// Test getpeername on a socket that has connected but not yet been accepted.
fn test_connected_before_accepted() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // the server address
    let mut server_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: 0u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: libc::INADDR_LOOPBACK.to_be(),
        },
        sin_zero: [0; 8],
    };

    // bind on the server address
    {
        let rv = unsafe {
            libc::bind(
                fd_server,
                &server_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                std::mem::size_of_val(&server_addr) as u32,
            )
        };
        assert_eq!(rv, 0);
    }

    // get the assigned port number
    {
        let mut server_addr_size = std::mem::size_of_val(&server_addr) as u32;
        let rv = unsafe {
            libc::getsockname(
                fd_server,
                &mut server_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                &mut server_addr_size as *mut libc::socklen_t,
            )
        };
        assert_eq!(rv, 0);
        assert_eq!(server_addr_size, std::mem::size_of_val(&server_addr) as u32);
    }

    // listen for connections
    {
        let rv = unsafe { libc::listen(fd_server, 0) };
        assert_eq!(rv, 0);
    }

    // connect to the server address
    {
        let rv = unsafe {
            libc::connect(
                fd_client,
                &server_addr as *const libc::sockaddr_in as *const libc::sockaddr,
                std::mem::size_of_val(&server_addr) as u32,
            )
        };
        assert_eq!(rv, 0);
    }

    // the sockaddr that we expect to have after calling getpeername()
    let expected_addr = server_addr;

    // client socket arguments for getpeername()
    let mut args = GetpeernameArguments {
        fd: fd_client,
        // fill the sockaddr with dummy data
        addr: Some(libc::sockaddr_in {
            sin_family: 123u16,
            sin_port: 456u16.to_be(),
            sin_addr: libc::in_addr {
                s_addr: 789u32.to_be(),
            },
            sin_zero: [1; 8],
        }),
        addr_len: Some(std::mem::size_of::<libc::sockaddr_in>() as u32),
    };

    test_utils::run_and_close_fds(&[fd_client, fd_server], || {
        check_getpeername_call(&mut args, None)
    })?;

    // check that the returned length is expected
    test_utils::result_assert_eq(
        args.addr_len.unwrap() as usize,
        std::mem::size_of_val(&args.addr.unwrap()),
        "Unexpected addr length",
    )?;

    // check that the returned server address is expected
    sockaddr_check_equal(
        &args.addr.unwrap(),
        &expected_addr,
        /* ignore_port= */ false,
    )?;

    Ok(())
}

/// Test getpeername using a socket connected on loopback.
fn test_connected_socket() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // the sockaddr that we expect to have after calling getpeername()
    let expected_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        // we don't know the port
        sin_port: 0u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: libc::INADDR_LOOPBACK.to_be(),
        },
        sin_zero: [0; 8],
    };

    // client arguments for getpeername()
    let mut args = GetpeernameArguments {
        fd: fd_client,
        // fill the sockaddr with dummy data
        addr: Some(libc::sockaddr_in {
            sin_family: 123u16,
            sin_port: 456u16.to_be(),
            sin_addr: libc::in_addr {
                s_addr: 789u32.to_be(),
            },
            sin_zero: [1; 8],
        }),
        addr_len: Some(std::mem::size_of::<libc::sockaddr_in>() as u32),
    };

    test_utils::run_and_close_fds(&[fd_client, fd_server, fd_accepted], || {
        check_getpeername_call(&mut args, None)
    })?;

    // check that the returned length is expected
    test_utils::result_assert_eq(
        args.addr_len.unwrap() as usize,
        std::mem::size_of_val(&args.addr.unwrap()),
        "Unexpected addr length",
    )?;

    // check that the returned client address is expected
    sockaddr_check_equal(
        &args.addr.unwrap(),
        &expected_addr,
        /* ignore_port= */ true,
    )?;

    // check that the port is valid
    test_utils::result_assert(args.addr.unwrap().sin_port > 0, "Unexpected port")?;

    Ok(())
}

/// Test getpeername using the server's accepted socket.
fn test_accepted_socket() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // the sockaddr that we expect to have after calling getpeername()
    let expected_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        // we don't know the port
        sin_port: 0u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: libc::INADDR_LOOPBACK.to_be(),
        },
        sin_zero: [0; 8],
    };

    // accepted socket arguments for getpeername()
    let mut args = GetpeernameArguments {
        fd: fd_accepted,
        // fill the sockaddr with dummy data
        addr: Some(libc::sockaddr_in {
            sin_family: 123u16,
            sin_port: 456u16.to_be(),
            sin_addr: libc::in_addr {
                s_addr: 789u32.to_be(),
            },
            sin_zero: [1; 8],
        }),
        addr_len: Some(std::mem::size_of::<libc::sockaddr_in>() as u32),
    };

    test_utils::run_and_close_fds(&[fd_client, fd_server, fd_accepted], || {
        check_getpeername_call(&mut args, None)
    })?;

    // check that the returned length is expected
    test_utils::result_assert_eq(
        args.addr_len.unwrap() as usize,
        std::mem::size_of_val(&args.addr.unwrap()),
        "Unexpected addr length",
    )?;

    // check that the returned server address is expected
    sockaddr_check_equal(
        &args.addr.unwrap(),
        &expected_addr,
        /* ignore_port= */ true,
    )?;

    // check that the port is valid
    test_utils::result_assert(args.addr.unwrap().sin_port > 0, "Unexpected port")?;

    Ok(())
}

/// Run getsockname on one fd and getpeername on another fd, and make sure they match.
fn compare_sockname_peername(
    fd_sockname: libc::c_int,
    fd_peername: libc::c_int,
) -> Result<(), String> {
    let mut sockname_addr = libc::sockaddr_in {
        sin_family: 123u16,
        sin_port: 456u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 789u32.to_be(),
        },
        sin_zero: [1; 8],
    };

    {
        let mut size = std::mem::size_of_val(&sockname_addr) as u32;
        let rv = unsafe {
            libc::getsockname(
                fd_sockname,
                &mut sockname_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                &mut size as *mut libc::socklen_t,
            )
        };
        assert_eq!(rv, 0);
        assert_eq!(size, std::mem::size_of_val(&sockname_addr) as u32);
    }

    let mut peername_addr = libc::sockaddr_in {
        sin_family: 321u16,
        sin_port: 654u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: 987u32.to_be(),
        },
        sin_zero: [2; 8],
    };

    {
        let mut size = std::mem::size_of_val(&peername_addr) as u32;
        let rv = unsafe {
            libc::getpeername(
                fd_peername,
                &mut peername_addr as *mut libc::sockaddr_in as *mut libc::sockaddr,
                &mut size as *mut libc::socklen_t,
            )
        };
        assert_eq!(rv, 0);
        assert_eq!(size, std::mem::size_of_val(&peername_addr) as u32);
    }

    sockaddr_check_equal(
        &sockname_addr,
        &peername_addr,
        /* ignore_port= */ false,
    )
}

/// Test that getpeername and getsockname return the same results for client/server sockets.
fn test_sockname_peername() -> Result<(), String> {
    let fd_client = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let fd_server = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    assert!(fd_client >= 0);
    assert!(fd_server >= 0);

    // connect the client fd to the server
    let fd_accepted = connect_helper(fd_client, fd_server);

    // compare getsockname on the first argument to getpeername on the second
    compare_sockname_peername(fd_client, fd_accepted)?;
    compare_sockname_peername(fd_accepted, fd_client)?;
    compare_sockname_peername(fd_server, fd_client)?;

    Ok(())
}

fn sockaddr_check_equal(
    a: &libc::sockaddr_in,
    b: &libc::sockaddr_in,
    ignore_port: bool,
) -> Result<(), String> {
    test_utils::result_assert_eq(a.sin_family, b.sin_family, "Unexpected family")?;
    if !ignore_port {
        test_utils::result_assert_eq(a.sin_port, b.sin_port, "Unexpected port")?;
    }
    test_utils::result_assert_eq(a.sin_addr.s_addr, b.sin_addr.s_addr, "Unexpected address")?;
    test_utils::result_assert_eq(a.sin_zero, b.sin_zero, "Unexpected padding")?;
    Ok(())
}

fn check_getpeername_call(
    args: &mut GetpeernameArguments,
    expected_errno: Option<libc::c_int>,
) -> Result<(), String> {
    // if the pointers will be non-null, make sure the length is not greater than the actual data size
    // so that we don't segfault
    if args.addr.is_some() && args.addr_len.is_some() {
        assert!(args.addr_len.unwrap() as usize <= std::mem::size_of_val(&args.addr.unwrap()));
    }

    // will modify args.addr and args.addr_len
    let rv = unsafe {
        libc::getpeername(
            args.fd,
            args.addr.as_mut_ptr() as *mut libc::sockaddr,
            args.addr_len.as_mut_ptr(),
        )
    };

    let errno = test_utils::get_errno();

    match expected_errno {
        // if we expect the socket() call to return an error (rv should be -1)
        Some(expected_errno) => {
            if rv != -1 {
                return Err(format!("Expecting a return value of -1, received {}", rv));
            }
            if errno != expected_errno {
                return Err(format!(
                    "Expecting errno {} \"{}\", received {} \"{}\"",
                    expected_errno,
                    test_utils::get_errno_message(expected_errno),
                    errno,
                    test_utils::get_errno_message(errno)
                ));
            }
        }
        // if no error is expected (rv should be 0)
        None => {
            if rv != 0 {
                return Err(format!(
                    "Expecting a return value of 0, received {} \"{}\"",
                    rv,
                    test_utils::get_errno_message(errno)
                ));
            }
        }
    }

    Ok(())
}
