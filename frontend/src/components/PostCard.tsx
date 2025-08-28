import React, { useState } from 'react';
import { Heart, MessageCircle, Share, MoreHorizontal } from 'lucide-react';
import { Post, PostService } from '../services/api';

interface PostCardProps {
  post: Post;
  onLikeToggle: (postId: string, newLikeCount: number, isLiked: boolean) => void;
}

export const PostCard: React.FC<PostCardProps> = ({ post, onLikeToggle }) => {
  const [isLiking, setIsLiking] = useState(false);

  const handleLike = async () => {
    if (isLiking) return;
    
    setIsLiking(true);
    try {
      const isLiked = await PostService.toggleLike(post.id);
      const newLikeCount = isLiked ? post.likes_count + 1 : post.likes_count - 1;
      onLikeToggle(post.id, newLikeCount, isLiked);
    } catch (error) {
      console.error('Error toggling like:', error);
    } finally {
      setIsLiking(false);
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffInHours = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60));
    
    if (diffInHours < 1) return 'hace unos minutos';
    if (diffInHours < 24) return `hace ${diffInHours}h`;
    return date.toLocaleDateString();
  };

  const getInitials = (name: string) => {
    return name.charAt(0).toUpperCase();
  };

  return (
    <div className="card-pitaia mb-6">
      <div className="flex items-start space-x-4">
        {/* Avatar */}
        <div className="w-12 h-12 bg-gradient-to-r from-pitaia-red-500 to-pitaia-green-500 rounded-full flex items-center justify-center flex-shrink-0">
          <span className="text-white font-semibold">
            {getInitials(post.display_name || post.username)}
          </span>
        </div>

        <div className="flex-1 min-w-0">
          {/* Header */}
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center space-x-2">
              <h4 className="font-semibold text-pitaia-gray-900 truncate">
                {post.display_name || post.username}
              </h4>
              <span className="text-pitaia-gray-600 text-sm">@{post.username}</span>
              <span className="text-pitaia-gray-400 text-sm">•</span>
              <span className="text-pitaia-gray-400 text-sm">{formatDate(post.created_at)}</span>
            </div>
            <button className="text-pitaia-gray-400 hover:text-pitaia-gray-600 p-1">
              <MoreHorizontal className="w-4 h-4" />
            </button>
          </div>

          {/* Content */}
          <div className="mb-4">
            <p className="text-pitaia-gray-700 whitespace-pre-wrap">{post.content}</p>
            {post.image_url && (
              <div className="mt-3">
                <img 
                  src={post.image_url} 
                  alt="Post image" 
                  className="rounded-xl max-w-full h-auto"
                />
              </div>
            )}
          </div>

          {/* Actions */}
          <div className="flex items-center space-x-6 text-sm text-pitaia-gray-600">
            <button 
              onClick={handleLike}
              disabled={isLiking}
              className={`flex items-center space-x-2 hover:text-pitaia-red-600 transition-colors ${
                post.is_liked ? 'text-pitaia-red-600' : ''
              } ${isLiking ? 'opacity-50' : ''}`}
            >
              <Heart className={`w-5 h-5 ${post.is_liked ? 'fill-current' : ''}`} />
              <span>{post.likes_count}</span>
            </button>
            
            <button className="flex items-center space-x-2 hover:text-pitaia-green-600 transition-colors">
              <MessageCircle className="w-5 h-5" />
              <span>{post.comments_count}</span>
            </button>
            
            <button className="flex items-center space-x-2 hover:text-pitaia-gray-800 transition-colors">
              <Share className="w-5 h-5" />
              <span>Compartir</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
