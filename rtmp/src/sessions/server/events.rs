use std::time::{SystemTime};
use rml_amf0::Amf0Value;
use ::time::RtmpTimestamp;
use ::sessions::StreamMetadata;

/// An event that a server session can raise
pub enum ServerSessionEvents {
    /// The client is changing the maximum size of the RTMP chunks they will be sending
    ClientChunkSizeChanged {
        new_chunk_size: u32,
    },

    /// The client is requesting a connection on the specified RTMP application name
    ConnectionRequested {
        request_id: u32,
        app_name: String,
    },

    /// The client is requesting a stream key be released for use.
    ReleaseStreamRequested {
        request_id: u32,
        app_name: String,
        stream_key: String,
    },

    /// The client is requesting the ability to publish on the specified stream key,
    PublishStreamRequested {
        request_id: u32,
        app_name: String,
        stream_key: String,
        stream_id: u32,
    },

    /// The client is finished publishing on the specified stream key
    PublishStreamFinished {
        app_name: String,
        stream_key: String,
    },

    /// The client is changing metadata properties of the stream being published
    StreamMetadataChanged {
        app_name: String,
        stream_key: String,
        metadata: StreamMetadata,
    },

    /// Audio data was received from the client
    AudioDataReceived {
        app_name: String,
        stream_key: String,
        data: Vec<u8>,
        timestamp: RtmpTimestamp,
        received_at: SystemTime,
    },

    /// Video data received from the client
    VideoDataReceived {
        app_name: String,
        stream_key: String,
        data: Vec<u8>,
        timestamp: RtmpTimestamp,
        received_at: SystemTime,
    },

    /// The client sent an Amf0 command that was not able to be handled
    UnhandleableAmf0Command {
        command_name: String,
        transaction_id: u32,
        command_object: Amf0Value,
        additional_values: Vec<Amf0Value>,
    },

    /// The client is requesting playback of the specified stream
    PlayStreamRequested {
        request_id: u32,
        app_name: String,
        stream_key: String,
        //video_type
        start_at: u32,
        duration: u32,
        reset: bool,
        stream_id: u32,
    },

    /// The client is finished with playback of the specified stream
    PlayStreamFinished {
        app_name: String,
        stream_key: String,
    },

    /// The client has sent an acknowledgement that they have received the specified number of bytes
    AcknowledgementReceived {
        bytes_received: u32,
    },

    /// The client has responded to a ping request
    PingResponseReceived {
        timestamp: RtmpTimestamp,
    },

    /// The server has sent a ping request to the client.
    PingRequestSent {
        timestamp: RtmpTimestamp,
    }
}