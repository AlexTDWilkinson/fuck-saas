            // Get DOM elements
            const dropZone = document.getElementById('drop-zone');
            const messageInput = document.getElementById('message-input');

            // Get channel and user info from window globals 
            const channelId = window.CHANNEL_ID;
            const userId = window.USER_ID;
            const userName = window.USERNAME;

            // Scroll to bottom of chat on load
            const messagesContainer = document.querySelector('.messages');
            // Wait for content to load before scrolling
            window.addEventListener('load', () => {
                requestAnimationFrame(() => {
                    messagesContainer.scrollTop = messagesContainer.scrollHeight;
                });
            });

            // Message sending
            async function sendMessage() {
                const message = messageInput.value.trim();
                if (!message) return;
                const now = Math.floor(new Date().getTime() + new Date().getTimezoneOffset() * 60000); // UTC timestamp in ms

                const tempMessage = {
                    content: message,
                    creator_id: userId,
                    username: window.USERNAME,
                    created_at: now,
                    edited_at: null,
                    pending: true
                };

                // Clear input immediately
                messageInput.value = '';

                // Add message to UI immediately
                const messagesContainer = document.querySelector('.messages');
                const messageHtml = formatMessage({...tempMessage});
                messagesContainer.insertAdjacentHTML('beforeend', messageHtml);
                messagesContainer.scrollTop = messagesContainer.scrollHeight;

                try {
                    const response = await fetch('/api/messages', {
                        method: 'POST',
                        headers: {'Content-Type': 'application/json'},
                        body: JSON.stringify({
                            channel_id: channelId,
                            content: message
                        })
                    });

                    if (!response.ok) throw new Error('Failed to send message');
                    
                    // Message will be confirmed via normal polling
                } catch (error) {
                    console.error('Error sending message:', error);
                    // Remove temporary message if request failed
                    const contentHash = message.content.split('').reduce((hash, char) => ((hash << 5) - hash) + char.charCodeAt(0), 0).toString(36);
                    document.querySelector(`[data-content-hash="${contentHash}"]`)?.remove();
                    
                    // Show alert to user
                    const alert = document.createElement('div');
                    alert.style.cssText = `
                        position: fixed;
                        bottom: 80px;
                        right: 20px;
                        background: #ff4444;
                        color: white;
                        padding: 10px 20px;
                        border-radius: 4px;
                        box-shadow: 0 2px 5px rgba(0,0,0,0.2);
                    `;
                    alert.textContent = 'Failed to send message';
                    document.body.appendChild(alert);
                    setTimeout(() => alert.remove(), 5000);
                }
            }

            // File upload handling
            function toggleDropZone() {
                dropZone.style.display = dropZone.style.display === 'none' ? 'block' : 'none';
            }

            function handleDragEvent(e, highlight) {
                e.preventDefault();
                e.stopPropagation();
                dropZone.style.borderColor = highlight ? 'var(--accent-color)' : 'var(--border-color)';
            }

            dropZone.addEventListener('dragover', e => handleDragEvent(e, true));
            dropZone.addEventListener('dragleave', e => handleDragEvent(e, false));
            dropZone.addEventListener('drop', e => {
                handleDragEvent(e, false);
                const files = e.dataTransfer.files;
                if (!files.length) return;
                // TODO: Implement file upload
                console.log('Files dropped:', files);
            });

            async function deleteMessage(timestamp) {
                // Convert string timestamp to number if needed
                const numericTimestamp = typeof timestamp === 'string' ? parseInt(timestamp) : timestamp;
                
                try {
                    const response = await fetch('/api/messages/delete', {
                        method: 'POST',
                        headers: {'Content-Type': 'application/json'},
                        body: JSON.stringify({
                            channel_id: channelId,
                            created_at: numericTimestamp
                        })
                    });

                    if (!response.ok) return;
                    document.getElementById(timestamp).remove();
                } catch (error) {
                    console.error('Error deleting message:', error);
                }
            }

            // Polling configuration
            let pollInterval = 2000; // Start with 2 seconds
            const MIN_POLL_INTERVAL = 2000; // Minimum 2 seconds
            const MAX_POLL_INTERVAL = 30000; // Maximum 30 seconds
            let lastMessageTimestamp = null;
            let consecutiveEmptyPolls = 0;
            let isInitialLoad = true;

            function formatTimestamp(timestamp) {
                // Server sends seconds, no need to multiply
             
                const now = new Date(Date.now() + new Date().getTimezoneOffset() * 60000); // UTC now


                return now.toLocaleDateString([], {
                    month: 'short',
                    day: '2-digit',
                    year: 'numeric',
                    hour: '2-digit',
                    minute: '2-digit'
                });
               
            }

            // Function to clean content for safe storage and display
            function cleanContent(content) {
                // First escape backslashes to prevent double escaping
                let cleaned = content.replace(/\\/g, "\\\\");
                
                // Then handle quotes and HTML characters
                cleaned = cleaned
                    .replace(/'/g, "\\'")
                    .replace(/"/g, "&quot;")
                    .replace(/</g, "&lt;")
                    .replace(/>/g, "&gt;");

                // Handle newlines and tabs that could break HTML formatting
                cleaned = cleaned
                    .replace(/\n/g, "<br>")
                    .replace(/\t/g, "&nbsp;&nbsp;&nbsp;&nbsp;");

                return cleaned;
            }

            // Function to format messages
            function formatMessage(message) {
                const {content, creator_id, username, created_at, edited_at, pending} = message;
                const cleaned_safe_content = cleanContent(content);
                const contentHash = message.content.split('').reduce((hash, char) => ((hash << 5) - hash) + char.charCodeAt(0), 0).toString(36);
      
                const messageActions = creator_id === userId ? `
                    <button 
                        onclick="editMessage('${created_at}', '${cleaned_safe_content}')"
                        class="button"
                        style="font-size: 0.8rem; padding: 0.2rem 0.5rem; background-color: #007bff; color: white; transition: background-color 0.2s; cursor: pointer;"
                        onmouseover="this.style.backgroundColor='#0056b3'"
                        onmouseout="this.style.backgroundColor='#007bff'"
                    >
                        Edit
                    </button>
                    <button 
                        onclick="deleteMessage('${created_at}')"
                        class="button"
                        style="font-size: 0.8rem; padding: 0.2rem 0.5rem; background-color: #dc3545; color: white; transition: background-color 0.2s; cursor: pointer;"
                        onmouseover="this.style.backgroundColor='#c82333'"
                        onmouseout="this.style.backgroundColor='#dc3545'"
                    >
                        Delete
                    </button>
                ` : '';
                const timeDisplay = edited_at ? 
                    `${formatTimestamp(edited_at)} (edited)` : 
                    formatTimestamp(created_at);
                const pendingIndicator = pending ? `
                    <span class="pending-indicator" style="
                        display: inline-block;
                        margin-left: 0.5rem;
                        font-size: 0.8rem;
                        color: var(--text-secondary);
                    ">
                        <span class="dot" style="
                            display: inline-block;
                            animation: pulse 1.4s infinite;
                            margin-right: 2px;
                        ">•</span>
                        <span class="dot" style="
                            display: inline-block;
                            animation: pulse 1.4s infinite 0.2s;
                            margin-right: 2px;
                        ">•</span>
                        <span class="dot" style="
                            display: inline-block;
                            animation: pulse 1.4s infinite 0.4s;
                        ">•</span>
                        <style>
                            @keyframes pulse {
                                0%, 80%, 100% { opacity: 0.3; }
                                40% { opacity: 1; }
                            }
                        </style>
                    </span>
                ` : '';
                
                return `
                    <article id="${created_at}"${pending ? ` data-content-hash="${contentHash}"` : ''}>
                        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                            <strong>${username}</strong>
                            ${messageActions}
                            <span style="color: var(--text-secondary); font-size: 0.8rem;">
                                ${timeDisplay}
                            </span>
                            ${pendingIndicator}
                        </div>
                        <p>${content}</p>
                    </article>
                `;
            }

            // Function to check if scrolled to bottom
            function isScrolledToBottom(element) {
                const threshold = 50; // pixels from bottom to consider "at bottom"
                return element.scrollHeight - element.scrollTop - element.clientHeight < threshold;
            }

            // Function to show new message notification
            function showNewMessageAlert() {
                const alert = document.createElement('div');
                alert.style.cssText = `
                    position: fixed;
                    bottom: 10%;
                    left: 50%;
                    transform: translateX(-50%);
                    background: #4CAF50;
                    color: white;
                    padding: 10px 20px;
                    border-radius: 4px;
                    cursor: pointer;
                    box-shadow: 0 8px 24px rgba(0,0,0,0.4);
                    animation: bounceUpDown 1s ease-in-out infinite, fadeIn 0.3s ease-in;
                `;
                alert.textContent = 'New messages ↓';
                alert.onclick = () => {
                    const messagesContainer = document.querySelector('.messages');
                    messagesContainer.scrollTop = messagesContainer.scrollHeight;
                    alert.remove();
                };
                
                // Add animation keyframes
                const style = document.createElement('style');
                style.textContent = `
                    @keyframes bounceUpDown {
                        0%, 100% { transform: translateX(-50%) translateY(0); }
                        50% { transform: translateX(-50%) translateY(-10px); }
                    }
                    @keyframes fadeIn {
                        from { opacity: 0; transform: translateX(-50%) translateY(20px); }
                        to { opacity: 1; transform: translateX(-50%) translateY(0); }
                    }
                `;
                document.head.appendChild(style);
                document.body.appendChild(alert);
            }

            function handleNewMessages(data, messagesContainer) {
                if (!data.messages?.length) {
                    consecutiveEmptyPolls++;
                    if (consecutiveEmptyPolls <= 2) return;
                    pollInterval = Math.min(pollInterval * 1.5, MAX_POLL_INTERVAL);
                    return;
                }

                pollInterval = MIN_POLL_INTERVAL;
                consecutiveEmptyPolls = 0;
                
                const wasAtBottom = isScrolledToBottom(messagesContainer);
                const previousLastTimestamp = lastMessageTimestamp;
                let hasNewMessages = false;
                
                data.messages.forEach(message => {
                    // Match temporary messages by content hash
                    const contentHash = message.content.split('').reduce((hash, char) => ((hash << 5) - hash) + char.charCodeAt(0), 0).toString(36);
                    const tempMessage = document.querySelector(`[data-content-hash="${contentHash}"]`);
                    if (tempMessage) {
                        tempMessage.remove();
                    }

                    // Update or add message
                    const existingMessage = document.getElementById(message.created_at);
                    if (existingMessage) {
                        existingMessage.outerHTML = formatMessage(message);
                    } else {
                        hasNewMessages = true;
                        messagesContainer.insertAdjacentHTML('beforeend', formatMessage(message));
                    }

                    // Update last message timestamp
                    if (message.created_at > (lastMessageTimestamp || 0)) {
                        lastMessageTimestamp = message.created_at;
                    }
                });
                
                // Show new message alert only if:
                // 1. We're not at the bottom
                // 2. Not initial load
                // 3. Actually received new messages
                // 4. No existing alert is present
                if (!wasAtBottom && !isInitialLoad && hasNewMessages && !document.querySelector('[data-new-message-alert]')) {
                    showNewMessageAlert();
                } else if (wasAtBottom && hasNewMessages) {
                    // If we're at the bottom and got new messages, scroll to bottom
                    messagesContainer.scrollTop = messagesContainer.scrollHeight;
                }
            }

            // Function to fetch new messages
            async function pollMessages() {
                try {
                    const response = await fetch(`/api/messages/poll?channel_id=${channelId}&last_timestamp=${lastMessageTimestamp || ''}`);
                    if (!response.ok) throw new Error('Poll failed');
                    
                    const data = await response.json();
                    const messagesContainer = document.querySelector('.messages');
                    handleNewMessages(data, messagesContainer);
                    
                    if (isInitialLoad) {
                        isInitialLoad = false;
                    }
                } catch (error) {
                    console.error('Polling error:', error);
                    pollInterval = Math.min(pollInterval * 2, MAX_POLL_INTERVAL);
                }
                
                setTimeout(pollMessages, pollInterval);
            }

            function resetPollInterval() {
                pollInterval = MIN_POLL_INTERVAL;
                consecutiveEmptyPolls = 0;
            }

            // Start polling when page loads
            pollMessages();

            // Reset poll interval on user activity
            document.addEventListener('keydown', resetPollInterval);
            messageInput.addEventListener('focus', resetPollInterval);

            // Add after deleteMessage function
            async function editMessage(timestamp, currentContent) {
                const newContent = prompt('Edit message:', currentContent);
                if (!newContent || newContent === currentContent) return;

                // Convert string timestamp to number if needed
                const numericTimestamp = typeof timestamp === 'string' ? parseInt(timestamp) : timestamp;

                try {
                    const response = await fetch('/api/messages/edit', {
                        method: 'POST',
                        headers: {'Content-Type': 'application/json'},
                        body: JSON.stringify({
                            channel_id: channelId,
                            created_at: numericTimestamp,
                            content: newContent
                        })
                    });

                    if (!response.ok) throw new Error('Failed to edit message');
                    
                    // Update message immediately in UI
                    const existingMessage = document.getElementById(timestamp);
                    if (existingMessage) {
                        const updatedMessage = {
                            content: newContent,
                            creator_id: userId,
                            username: window.USERNAME,
                            created_at: numericTimestamp,
                            edited_at: Math.floor(Date.now() / 1000)
                        };
                        existingMessage.outerHTML = formatMessage(updatedMessage);
                    }

                } catch (error) {
                    console.error('Error editing message:', error);
                    
                    // Show error alert
                    const alert = document.createElement('div');
                    alert.style.cssText = `
                        position: fixed;
                        bottom: 80px;
                        right: 20px;
                        background: #ff4444;
                        color: white;
                        padding: 10px 20px;
                        border-radius: 4px;
                        box-shadow: 0 2px 5px rgba(0,0,0,0.2);
                    `;
                    alert.textContent = 'Failed to edit message';
                    document.body.appendChild(alert);
                    setTimeout(() => alert.remove(), 5000);
                }
            }